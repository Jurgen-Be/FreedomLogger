/// Detailed pattern formatter for FreedomLogger
///
/// Implements detailed log format: [TIMESTAMP] [FILE:LINE] LEVEL: MESSAGE
/// This adds source file and line number information to help with debugging.
/// Useful for development and detailed production logging.
///
/// Example output: [2025-09-06 15:30:45] [main.rs:42] INFO: User logged in

use super::basic::LogInfo;
use crate::core::config::LogLevel;

/// Format a log message using the Detailed pattern
///
/// Detailed pattern format: [TIMESTAMP] [FILE:LINE] LEVEL: MESSAGE
/// If file/line information is not available, falls back to basic pattern
///
/// # Arguments
/// * `info` - All log information (uses message, level, timestamp, file, line)
///
/// # Returns
/// Formatted string ready to write to log file
///

pub fn format_detailed(info: &LogInfo) -> String {
    match (info.file, info.line) {
        // Both file and line available - full detailed format
        (Some(file), Some(line)) => {
            format!(
                "[{}] [{}:{}] {}: {}",
                info.timestamp,
                file,
                line,
                info.level.as_str(),
                info.message
            )
        }

        // Only file available - show file without line
        (Some(file), None) => {
            format!(
                "[{}] [{}] {}: {}",
                info.timestamp,
                file,
                info.level.as_str(),
                info.message
            )
        }

        // No file/line info available - fallback to basic format
        (None, _) => {
            format!(
                "[{}] {}: {}",
                info.timestamp,
                info.level.as_str(),
                info.message
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detailed_formatting_with_full_location() {
        let info = LogInfo::new(
            "Test message",
            LogLevel::Info,
            "2025-09-06 15:30:45"
        ).with_location("main.rs", 42);

        let result = format_detailed(&info);
        assert_eq!(result, "[2025-09-06 15:30:45] [main.rs:42] INFO: Test message");
    }

    #[test]
    fn test_detailed_formatting_with_file_only() {
        let mut info = LogInfo::new(
            "Test message",
            LogLevel::Warning,
            "2025-09-06 15:30:45"
        );
        info.file = Some("utils.rs");
        // line remains None

        let result = format_detailed(&info);
        assert_eq!(result, "[2025-09-06 15:30:45] [utils.rs] WARNING: Test message");
    }

    #[test]
    fn test_detailed_formatting_fallback_to_basic() {
        // No file/line info - should fallback to basic format
        let info = LogInfo::new(
            "Test message",
            LogLevel::Error,
            "2025-09-06 15:30:45"
        );

        let result = format_detailed(&info);
        assert_eq!(result, "[2025-09-06 15:30:45] ERROR: Test message");
    }

    #[test]
    fn test_different_log_levels() {
        let info = LogInfo::new(
            "Debug info",
            LogLevel::Debug,
            "2025-09-06 15:30:45"
        ).with_location("debug.rs", 123);

        let result = format_detailed(&info);
        assert_eq!(result, "[2025-09-06 15:30:45] [debug.rs:123] DEBUG: Debug info");
    }
}