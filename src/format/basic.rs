// File: src/format/basic.rs

/// Basic pattern formatter for FreedomLogger
///
/// Implements the simplest log format: [TIMESTAMP] LEVEL: MESSAGE
/// This is used as the default/fallback pattern when other patterns fail
/// or when users want clean, simple logging output.
///
/// Example output: [2025-09-06 15:30:45] INFO: User logged in

use crate::core::config::LogLevel;

/// Information needed to format any log message
/// This struct contains all posible data that formatters might need

#[derive(Debug)]
pub struct LogInfo<'a> {
    // The log messsage
    pub message: &'a str,
    // Log level
    pub level: LogLevel,
    // Timestamp
    pub timestamp: &'a str,
    // File name
    pub file: Option<&'a str>,
    // Line nr
    pub line: Option<u32>,
    // Thread
    pub thread: Option<&'a str>,
}

impl<'a> LogInfo<'a> {
    /// Create new LogInfo with required fields
    /// Optional fields (file, line, thread) can be set separately

    pub fn new(message: &'a str, level: LogLevel, timestamp: &'a str) -> Self {
        Self {
            message,
            level,
            timestamp,
            file: None,
            line: None,
            thread: None,
        }
    }


    /// Add file and line information (used by detailled patterns)

    pub fn with_location(mut self, file: &'a str, line: u32) -> Self {
        self.file = Some(file);
        self.line = Some(line);
        self
    }


    /// Add thread information (Used by extended patterns)
    pub fn with_thread(mut self, thread: &'a str) -> Self {
        self.thread = Some(thread);
        self
    }
}


/// Format a log message using the Basic pattern
///
/// Basic pattern format: [TIMESTAMP] LEVEL: MESSAGE
///
/// # Arguments
/// * `info` - All log information (only uses message, level, timestamp)
///
/// # Returns
/// Formatted string ready to write to log file
///
///
pub fn format_basic(info: &LogInfo) -> String {
    format!(
        "[{}] {}: {}",
        info.timestamp,
        info.level.as_str(),
        info.message
    )
}


/*
TESTS FOR THE BASIC LOGGER
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_formatting() {
        let info = LogInfo::new(
            "Test message",
            LogLevel::Info,
            "2025-09-06 15:30:45"
        );

        let result = format_basic(&info);
        assert_eq!(result, "[2025-09-06 15:30:45] INFO: Test message");
    }

    #[test]
    fn test_different_log_levels() {
        let timestamp = "2025-09-06 15:30:45";
        let message = "Test message";

        let error_info = LogInfo::new(message, LogLevel::Error, timestamp);
        assert_eq!(format_basic(&error_info), "[2025-09-06 15:30:45] ERROR: Test message");

        let debug_info = LogInfo::new(message, LogLevel::Debug, timestamp);
        assert_eq!(format_basic(&debug_info), "[2025-09-06 15:30:45] DEBUG: Test message");
    }

    #[test]
    fn test_loginfo_builder() {
        let info = LogInfo::new("Test", LogLevel::Warning, "2025-09-06 15:30:45")
            .with_location("main.rs", 42)
            .with_thread("main");

        // Basic formatter ignores file/line/thread info
        let result = format_basic(&info);
        assert_eq!(result, "[2025-09-06 15:30:45] WARNING: Test");
    }
}