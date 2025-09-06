/// File: src/core/config.rs

/*
Configuration types and structures for FreedomLogger
This module defines the core configuration that determines how the logger behaves,
what pattern to use, witch levels to log, file paths, etc.

The configuration is set once during the initialization and remains constant.
 */

use std::path::PathBuf;

/*
Log levels in order from most critical to the least critical
Used for filtering - if logger is configured with INFO level.
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 1,
    Warning = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl LogLevel {
    /// Convert log level to string for output formatting
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warning => "WARNING",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }

    /// Check if this level should be logged given the configured minimum level
    ///
    /// # Arguments
    /// * 'configured level' - The minimum level configured during init
    ///
    /// # Returns
    /// True if this message should be logged, false if it should be filtered.

    pub fn should_log(&self, configured_level: LogLevel) -> bool {
        *self <= configured_level
    }
}

/// Log formatting patterns from basic to advanced
/// Patterns determibe how log messages are formatted in the output file
/// Order goed from simple to complex

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Basic,
    Detailed,
    Extended,
    Json,
    Custom(String),
}

impl Pattern {
    /// Get the default pattern (Basic)
    /// Used as fallback when user-provided pattern is invalid

    pub fn default() -> Self {
        Pattern::Basic
    }

    /// Validate a custom pattern string
    /// Returns true if the pattern contains requires placeholders

    pub fn validate_custom(pattern: &str) -> bool {
        // Custom pattern must at least have {message} placeholder
        // Other placeholders like {timestamp}, {level} are optional
        pattern.contains("{message}")
        }
}

/// Complete logger configuration
///
/// Contains all settings needed to initialize the logger.
/// Created during init()

#[derive(Debug, Clone)]
pub struct LoggerConfig {
    // Pattern for formatting log messages
    pub pattern: Pattern,

    // Directory path where log files will be created
    pub file_path: PathBuf,

    // Base filename for log files
    pub file_name: String,

    // Minimum log level
    // None means log everything
    pub log_level: Option<LogLevel>,

    // Maximum file size before rotation 9in bytes)
    // Default: 10MB
    pub max_file_size: u64,

    // Maximum number of log files to keep
    // Default: 5
    pub max_backup_files: u32,
}

impl LoggerConfig {
    /// Create basic logger configuration (for logger::init)
    /// Uses default rotatiob settings: 10MB files, 5 backups
    /// Logs everything (no level filtering)

    pub fn basic(
        pattern: Pattern,
        file_path: PathBuf,
        file_name: String,
    ) -> Self {
        Self {
            pattern,
            file_path,
            file_name,
            log_level: None,
            max_file_size: 10 * 1024 * 1024,
            max_backup_files: 5,
        }
    }

    /// Create logger configuration with level filtering
    /// Uses default rotation settings: 10MB, 5 backups


}
