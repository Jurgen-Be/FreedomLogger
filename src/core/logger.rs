/// Main logger implementation for FreedomLogger
///
/// This is the core orchestrator that brings together all components:
/// - Configuration management
/// - Message formatting using patterns
/// - Writing to files using appropriate writers
/// - Log rotation when files get too large
/// - Error handling and fallback mechanisms
///
/// The Logger maintains internal state and provides the main logging methods
/// that users call: info(), debug(), error(), warning(), trace().

use std::sync::Mutex;
use std::thread;
use crate::core::config::{LoggerConfig, LogLevel, Pattern};
use crate::core::writers::{TextWriter, JsonWriter};
use crate::format::LogInfo;
use crate::rotation::{SizeBasedRotation, RotationResult};
use crate::error::{write_error_to_log, LoggerError};

/// Main logger struct that handles all logging operations
///
/// Contains configuration, writers, and rotation management.
/// Thread-safe through internal mutex for concurrent access.
pub struct Logger {
    /// Logger configuration (pattern, paths, levels, etc.)
    config: LoggerConfig,
    /// Text file writer for plain text logs
    text_writer: TextWriter,
    /// JSON writer for structured logs
    json_writer: JsonWriter,
    /// Log rotation manager
    rotation: SizeBasedRotation,
    /// Mutex for thread-safe logging operations
    write_mutex: Mutex<()>,
}

impl Logger {
    /// Create a new logger instance with the given configuration
    ///
    /// # Arguments
    /// * `config` - Complete logger configuration
    ///
    /// # Returns
    /// New Logger instance ready for logging operations
    pub fn new(config: LoggerConfig) -> Self {
        let rotation = SizeBasedRotation::new(
            config.max_file_size,
            config.max_backup_files,
        );

        Self {
            config,
            text_writer: TextWriter::new(),
            json_writer: JsonWriter::new(),
            rotation,
            write_mutex: Mutex::new(()),
        }
    }

    /// Log an ERROR level message
    ///
    /// # Arguments
    /// * `message` - The message to log
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message, file!(), line!());
    }

    /// Log a WARNING level message
    ///
    /// # Arguments
    /// * `message` - The message to log
    pub fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message, file!(), line!());
    }

    /// Log an INFO level message
    ///
    /// # Arguments
    /// * `message` - The message to log
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message, file!(), line!());
    }

    /// Log a DEBUG level message
    ///
    /// # Arguments
    /// * `message` - The message to log
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message, file!(), line!());
    }

    /// Log a TRACE level message
    ///
    /// # Arguments
    /// * `message` - The message to log
    pub fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message, file!(), line!());
    }

    /// Internal logging method that handles all log levels
    ///
    /// This method orchestrates the entire logging process:
    /// 1. Check if level should be logged (filtering)
    /// 2. Create LogInfo with current timestamp and location
    /// 3. Check and perform log rotation if needed
    /// 4. Format message using configured pattern
    /// 5. Write to appropriate file format
    /// 6. Handle any errors silently
    fn log(&self, level: LogLevel, message: &str, file: &str, line: u32) {
        // Step 1: Check if this log level should be written
        if !self.config.should_log_level(level) {
            return; // Silently ignore - no error
        }

        // Step 2: Thread-safe logging operation
        let _lock = match self.write_mutex.lock() {
            Ok(lock) => lock,
            Err(_) => {
                // Mutex poisoned - attempt to continue without lock
                // In production, we prefer to log something rather than nothing
                self.handle_error(LoggerError::RotationFailed {
                    current_file: "mutex".to_string(),
                    backup_file: "poisoned".to_string(),
                    reason: "Mutex poisoned during logging".to_string(),
                });
                return;
            }
        };

        // Step 3: Create log info with all available data
        let timestamp = self.get_current_timestamp();
        let thread_name = self.get_current_thread_name();

        let log_info = LogInfo::new(message, level, &timestamp)
            .with_location(file, line)
            .with_thread(&thread_name);

        // Step 4: Get appropriate file path based on pattern
        let log_file_path = match self.config.pattern {
            Pattern::Json => {
                // JSON pattern uses .json extension
                self.config.file_path.join(format!("{}.json", self.config.file_name))
            }
            _ => {
                // All other patterns use .log extension
                self.config.get_log_file_path()
            }
        };

        // Step 5: Check and perform rotation if needed
        match self.rotation.check_and_rotate(&log_file_path) {
            RotationResult::Failed(error) => {
                self.handle_error(error);
                // Continue with logging even if rotation failed
            }
            _ => {
                // Rotation completed or not needed - continue normally
            }
        }

        // Step 6: Write the log entry
        self.write_log_entry(&log_info, &log_file_path);
    }

    /// Write a log entry using the appropriate writer and format
    fn write_log_entry(&self, log_info: &LogInfo, file_path: &std::path::Path) {
        match self.config.pattern {
            Pattern::Json => {
                // Use JSON writer for JSON pattern
                if let Err(error) = self.json_writer.write_log_entry(log_info, file_path) {
                    self.handle_error(error);
                }
            }
            _ => {
                // Use text writer for all other patterns
                let formatted_message = self.config.pattern.format(log_info);
                if let Err(error) = self.text_writer.write_message(&formatted_message, file_path) {
                    self.handle_error(error);
                }
            }
        }
    }

    /// Get current timestamp as string
    fn get_current_timestamp(&self) -> String {
        use chrono::{Local, DateTime};
        let now: DateTime<Local> = Local::now();
        now.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// Get current thread name or ID
    fn get_current_thread_name(&self) -> String {
        thread::current()
            .name()
            .unwrap_or("unnamed")
            .to_string()
    }

    /// Handle logger internal errors by writing to error log
    ///
    /// This method never panics or returns errors - it's the final fallback
    fn handle_error(&self, error: LoggerError) {
        // Write to error log in same directory as main log
        write_error_to_log(&error, &self.config.file_path);
    }
}

/// Thread-safe implementation - Logger can be shared between threads
unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}