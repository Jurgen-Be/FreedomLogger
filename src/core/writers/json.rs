/// JSON file writer for FreedomLogger
///
/// Handles writing structured log data to JSON format files (.json extension).
/// Each log entry is written as a single JSON object per line (JSONL format).
/// This format is ideal for log aggregation tools, databases, and structured analysis.
///
/// Future v2 enhancement: This writer will be extended to support database output
/// by converting the JSON structure to database inserts.

use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use crate::error::{LoggerError, LoggerResult};
use crate::format::LogInfo;
use crate::core::config::LogLevel;

/// JSON file writer for structured log output
///
/// Outputs each log entry as a JSON object with consistent field structure.
/// Uses JSONL format (one JSON object per line) for easy parsing by log processors.
#[derive(Debug)]
pub struct JsonWriter;

impl JsonWriter {
    /// Create a new JSON writer instance
    pub fn new() -> Self {
        Self
    }

    /// Write log information as JSON to the specified file
    ///
    /// Converts LogInfo into structured JSON and writes to file.
    /// Each log entry becomes one line of JSON for easy processing.
    ///
    /// # Arguments
    /// * `log_info` - Complete log information to convert to JSON
    /// * `file_path` - Full path to the JSON log file
    ///
    /// # Returns
    /// Ok(()) on success, LoggerError on failure
    pub fn write_log_entry(&self, log_info: &LogInfo, file_path: &Path) -> LoggerResult<()> {
        // Step 1: Convert LogInfo to JSON string
        let json_string = self.format_as_json(log_info);

        // Step 2: Ensure directory exists
        self.ensure_directory_exists(file_path)?;

        // Step 3: Open file in append mode
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .map_err(|_| LoggerError::FileCreationFailed {
                path: file_path.display().to_string(),
                reason: "Failed to open JSON file for writing".to_string(),
            })?;

        // Step 4: Write JSON line
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", json_string)
            .map_err(|_| LoggerError::DiskFull {
                path: file_path.display().to_string(),
                bytes_attempted: json_string.len() + 1,
            })?;

        // Step 5: Flush to ensure data is written
        writer.flush()
            .map_err(|_| LoggerError::DiskFull {
                path: file_path.display().to_string(),
                bytes_attempted: json_string.len() + 1,
            })?;

        Ok(())
    }

    /// Convert LogInfo to JSON string format
    ///
    /// Creates structured JSON with consistent field names for all log entries.
    /// Missing optional fields are represented as null in JSON.
    fn format_as_json(&self, log_info: &LogInfo) -> String {
        // Manual JSON construction to avoid external dependencies
        let mut json_parts = Vec::new();

        // Required fields
        json_parts.push(format!("\"timestamp\":\"{}\"", self.escape_json_string(log_info.timestamp)));
        json_parts.push(format!("\"level\":\"{}\"", log_info.level.as_str()));
        json_parts.push(format!("\"message\":\"{}\"", self.escape_json_string(log_info.message)));

        // Optional fields - include as null if not present
        match log_info.file {
            Some(file) => json_parts.push(format!("\"file\":\"{}\"", self.escape_json_string(file))),
            None => json_parts.push("\"file\":null".to_string()),
        }

        match log_info.line {
            Some(line) => json_parts.push(format!("\"line\":{}", line)),
            None => json_parts.push("\"line\":null".to_string()),
        }

        match log_info.thread {
            Some(thread) => json_parts.push(format!("\"thread\":\"{}\"", self.escape_json_string(thread))),
            None => json_parts.push("\"thread\":null".to_string()),
        }

        // Combine into final JSON object
        format!("{{{}}}", json_parts.join(","))
    }

    /// Escape special characters in JSON strings
    ///
    /// Handles quotes, newlines, and other characters that need escaping in JSON.
    fn escape_json_string(&self, input: &str) -> String {
        input
            .replace("\\", "\\\\")  // Escape backslashes first
            .replace("\"", "\\\"")  // Escape quotes
            .replace("\n", "\\n")   // Escape newlines
            .replace("\r", "\\r")   // Escape carriage returns
            .replace("\t", "\\t")   // Escape tabs
    }

    /// Ensure the directory for the JSON file exists
    ///
    /// Creates parent directories recursively if they don't exist.
    fn ensure_directory_exists(&self, file_path: &Path) -> LoggerResult<()> {
        if let Some(parent_dir) = file_path.parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)
                    .map_err(|_| LoggerError::DirectoryCreationFailed {
                        path: parent_dir.display().to_string(),
                        reason: "Failed to create parent directories for JSON file".to_string(),
                    })?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::LogInfo;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_write_basic_json_entry() {
        let temp_dir = tempdir().unwrap();
        let json_path = temp_dir.path().join("test.json");

        let log_info = LogInfo::new("Test message", LogLevel::Info, "2025-09-06 15:30:45");
        let writer = JsonWriter::new();

        let result = writer.write_log_entry(&log_info, &json_path);
        assert!(result.is_ok());

        let content = fs::read_to_string(&json_path).unwrap();
        assert!(content.contains("\"message\":\"Test message\""));
        assert!(content.contains("\"level\":\"INFO\""));
        assert!(content.contains("\"timestamp\":\"2025-09-06 15:30:45\""));
    }

    #[test]
    fn test_write_detailed_json_entry() {
        let temp_dir = tempdir().unwrap();
        let json_path = temp_dir.path().join("detailed.json");

        let log_info = LogInfo::new("Detailed test", LogLevel::Debug, "2025-09-06 15:30:45")
            .with_location("test.rs", 42)
            .with_thread("main");

        let writer = JsonWriter::new();
        writer.write_log_entry(&log_info, &json_path).unwrap();

        let content = fs::read_to_string(&json_path).unwrap();
        assert!(content.contains("\"file\":\"test.rs\""));
        assert!(content.contains("\"line\":42"));
        assert!(content.contains("\"thread\":\"main\""));
    }

    #[test]
    fn test_json_string_escaping() {
        let writer = JsonWriter::new();

        let result = writer.escape_json_string("Message with \"quotes\" and \n newline");
        assert_eq!(result, "Message with \\\"quotes\\\" and \\n newline");
    }

    #[test]
    fn test_multiple_json_entries() {
        let temp_dir = tempdir().unwrap();
        let json_path = temp_dir.path().join("multi.json");

        let writer = JsonWriter::new();

        let info1 = LogInfo::new("First message", LogLevel::Info, "2025-09-06 15:30:45");
        let info2 = LogInfo::new("Second message", LogLevel::Warning, "2025-09-06 15:30:46");

        writer.write_log_entry(&info1, &json_path).unwrap();
        writer.write_log_entry(&info2, &json_path).unwrap();

        let content = fs::read_to_string(&json_path).unwrap();
        let lines: Vec<&str> = content.trim().split('\n').collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("First message"));
        assert!(lines[1].contains("Second message"));
    }
}