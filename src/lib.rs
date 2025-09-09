/// FreedomLogger - A professional logging library for Rust
///
/// FreedomLogger provides clean, efficient logging with automatic rotation,
/// multiple output formats, and error-proof operation. Designed for both
/// development and production use.
///
/// Features:
/// - Multiple log levels (ERROR, WARNING, INFO, DEBUG, TRACE) with filtering
/// - Various output patterns (Basic, Detailed, Extended, JSON, Custom)
/// - Automatic log rotation based on file size
/// - Thread-safe concurrent logging
/// - No external dependencies (except chrono for timestamps)
/// - Error-proof operation (internal errors logged separately)
/// - Easy single-initialization API
/// - Flexible logging macros supporting formatted messages
///
/// Usage:
/// 1. Initialize logger once in main(): logger::init(pattern, path, filename)
/// 2. Log anywhere in your code:
///    - Simple: logger::info("message")
///    - Formatted: log_info!("User {} logged in", user_id)
/// 3. All configuration is done at initialization time

use std::sync::{Arc, Once};
use std::path::Path;

// Import all our modules
pub mod error;
pub mod core;
pub mod format;
pub mod rotation;

// Re-export main types for user convenience
pub use core::{LogLevel, Pattern, LoggerConfig, Logger};
pub use error::LoggerError;

/// Global logger instance - initialized once, used everywhere
static mut GLOBAL_LOGGER: Option<Arc<Logger>> = None;
static INIT_ONCE: Once = Once::new();

/// Initialize the global logger with basic configuration
///
/// This is the simplest initialization - logs all levels with default settings.
/// Uses 10MB max file size and keeps 5 backup files.
///
/// # Arguments
/// * `pattern` - Log formatting pattern (Basic, Detailed, etc.)
/// * `file_path` - Directory path where log files will be created
/// * `file_name` - Base name for log files (without extension)
///
/// # Panics
/// Panics if called more than once or if initialization fails
pub fn log_init<P: AsRef<Path>>(pattern: Pattern, file_path: P, file_name: &str) {
    let path_buf = file_path.as_ref().to_path_buf();
    let config = LoggerConfig::basic(pattern, path_buf, file_name.to_string());
    log_init_with_config(config);
}

/// Initialize the global logger with log level filtering
///
/// Logs only messages at or above the specified level.
/// Uses default rotation settings (10MB, 5 backups).
///
/// # Arguments
/// * `pattern` - Log formatting pattern
/// * `file_path` - Directory path where log files will be created
/// * `file_name` - Base name for log files (without extension)
/// * `log_level` - Minimum log level to write
///
/// # Panics
/// Panics if called more than once or if initialization fails
pub fn log_init_with_level<P: AsRef<Path>>(
    pattern: Pattern,
    file_path: P,
    file_name: &str,
    log_level: LogLevel
) {
    let path_buf = file_path.as_ref().to_path_buf();
    let config = LoggerConfig::with_level(pattern, path_buf, file_name.to_string(), log_level);
    log_init_with_config(config);
}

/// Initialize the global logger with custom rotation settings
///
/// Full control over all logging parameters including rotation behavior.
///
/// # Arguments
/// * `pattern` - Log formatting pattern
/// * `file_path` - Directory path where log files will be created
/// * `file_name` - Base name for log files (without extension)
/// * `log_level` - Minimum log level to write
/// * `max_file_size` - Maximum file size in bytes before rotation
/// * `max_backup_files` - Number of backup files to keep
///
/// # Panics
/// Panics if called more than once or if initialization fails
pub fn log_init_with_rotation<P: AsRef<Path>>(
    pattern: Pattern,
    file_path: P,
    file_name: &str,
    log_level: LogLevel,
    max_file_size: u64,
    max_backup_files: u32,
) {
    let path_buf = file_path.as_ref().to_path_buf();
    let config = LoggerConfig::with_rotation(
        pattern,
        path_buf,
        file_name.to_string(),
        log_level,
        max_file_size,
        max_backup_files
    );
    log_init_with_config(config);
}

/// Initialize with a complete configuration object
///
/// Internal method used by all public init functions.
/// Ensures thread-safe single initialization.
fn log_init_with_config(config: LoggerConfig) {
    INIT_ONCE.call_once(|| {
        let logger = Logger::new(config);
        unsafe {
            GLOBAL_LOGGER = Some(Arc::new(logger));
        }
    });
}

/// Get reference to the global logger instance
///
/// Returns the initialized logger or panics if not initialized.
/// This is used internally by the logging functions.
#[allow(static_mut_refs)]
fn get_logger() -> &'static Arc<Logger> {
    unsafe {
        GLOBAL_LOGGER
            .as_ref()
            .expect("Logger not initialized - call logger::init() first")
    }
}

/// Log an ERROR level message
///
/// Logs critical errors that indicate serious problems.
/// These messages are always written regardless of log level filtering.
///
/// # Arguments
/// * `message` - The error message to log
pub fn log_error(message: &str) {
    get_logger().error(message);
}

/// Log a WARNING level message
///
/// Logs warning messages that indicate potential issues.
/// Written when log level is WARNING or higher.
///
/// # Arguments
/// * `message` - The warning message to log
pub fn log_warning(message: &str) {
    get_logger().warning(message);
}

/// Log an INFO level message
///
/// Logs general information about application flow.
/// Written when log level is INFO or higher.
///
/// # Arguments
/// * `message` - The info message to log
pub fn log_info(message: &str) {
    get_logger().info(message);
}

/// Log a DEBUG level message
///
/// Logs detailed information useful for debugging.
/// Written when log level is DEBUG or higher.
///
/// # Arguments
/// * `message` - The debug message to log
pub fn log_debug(message: &str) {
    get_logger().debug(message);
}

/// Log a TRACE level message
///
/// Logs very detailed trace information.
/// Written when log level is TRACE (logs everything).
///
/// # Arguments
/// * `message` - The trace message to log
pub fn log_trace(message: &str) {
    get_logger().trace(message);
}

// ============================================================================
// MACROS VOOR FORMATTED LOGGING
// ============================================================================

/// Macro for logging ERROR messages with formatting support
///
/// Supports both simple messages and formatted strings with arguments.
/// Uses Rust's built-in format! macro for automatic type handling.
///

#[macro_export]
macro_rules! log_error {
    // Simple message zonder formatting
    ($msg:expr) => {
        $crate::log_error($msg);
    };

    // Formatted message met argumenten
    ($fmt:expr, $($arg:expr),+ $(,)?) => {
        $crate::log_error(&format!($fmt, $($arg),+));
    };
}

/// Macro for logging WARNING messages with formatting support
///
/// Supports both simple messages and formatted strings with arguments.
///

/// ```
#[macro_export]
macro_rules! log_warning {
    ($msg:expr) => {
        $crate::log_warning($msg);
    };

    ($fmt:expr, $($arg:expr),+ $(,)?) => {
        $crate::log_warning(&format!($fmt, $($arg),+));
    };
}

/// Macro for logging INFO messages with formatting support
///
/// Supports both simple messages and formatted strings with arguments.
///

#[macro_export]
macro_rules! log_info {
    ($msg:expr) => {
        $crate::log_info($msg);
    };

    ($fmt:expr, $($arg:expr),+ $(,)?) => {
        $crate::log_info(&format!($fmt, $($arg),+));
    };
}

/// Macro for logging DEBUG messages with formatting support
///
/// This macro solves the original problem! It supports both simple messages
/// and formatted strings with arguments, automatically handling any type
/// that implements Display or Debug.
///

#[macro_export]
macro_rules! log_debug {
    ($msg:expr) => {
        $crate::log_debug($msg);
    };

    ($fmt:expr, $($arg:expr),+ $(,)?) => {
        $crate::log_debug(&format!($fmt, $($arg),+));
    };
}

/// Macro for logging TRACE messages with formatting support
///
/// Supports both simple messages and formatted strings with arguments.
///

#[macro_export]
macro_rules! log_trace {
    ($msg:expr) => {
        $crate::log_trace($msg);
    };

    ($fmt:expr, $($arg:expr),+ $(,)?) => {
        $crate::log_trace(&format!($fmt, $($arg),+));
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_basic_logging_integration() {
        let temp_dir = tempdir().unwrap();

        // Initialize logger
        log_init(Pattern::Basic, temp_dir.path(), "test");

        // Test basic function logging
        log_info("Test info message");
        log_warning("Test warning message");
        log_error("Test error message");

        // Test the new macros with formatting
        let database_path = PathBuf::from("/var/lib/myapp/database.db");
        let user_id = 12345;
        let status = "active";

        // This should now work without any errors!
        log_debug!("Database path: {:?}", database_path);
        log_info!("User {} has status: {}", user_id, status);
        log_warning!("Processing {} items", 42);
        log_error!("Failed to connect to {}", "localhost:5432");

        // Test simple messages still work
        log_debug!("Simple debug message");
        log_info!("Simple info message");

        // Check that log file was created
        let log_file = temp_dir.path().join("test.log");
        assert!(log_file.exists());

        // Check log content
        let content = fs::read_to_string(&log_file).unwrap();

        // Test basic function messages
        assert!(content.contains("INFO: Test info message"));
        assert!(content.contains("WARNING: Test warning message"));
        assert!(content.contains("ERROR: Test error message"));

        // Test macro messages
        assert!(content.contains("Database path:"));
        assert!(content.contains("User 12345 has status: active"));
        assert!(content.contains("Processing 42 items"));
        assert!(content.contains("Failed to connect to localhost:5432"));
        assert!(content.contains("Simple debug message"));
    }
}