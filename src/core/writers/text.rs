/// Text file writer for FreedomLogger
///
/// Handles writing formatted log messages to plain text files (.log extension).
/// Responsible for:
/// - Creating directories if they don't exist
/// - Opening/creating log files in append mode
/// - Writing formatted strings to files
/// - Proper error handling and reporting
///
/// This writer outputs human-readable text logs suitable for viewing
/// in text editors or processing with standard Unix tools.

use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use crate::error::{LoggerError, LoggerResult};

/// Text file writer for plain text log output
///
/// Handles all aspects of writing to text log files including
/// directory creation, file management, and error handling.
#[derive(Debug)]
pub struct TextWriter;

impl TextWriter {
    /// Create a new text writer instance
    pub fn new() -> Self {
        Self
    }

    /// Write a formatted log message to the specified file
    ///
    /// This method handles the complete write process:
    /// 1. Ensure directory exists (create if needed)
    /// 2. Open/create log file in append mode
    /// 3. Write the message with newline
    /// 4. Flush to ensure data is written
    ///
    /// # Arguments
    /// * `message` - The fully formatted log message to write
    /// * `file_path` - Full path to the log file
    ///
    /// # Returns
    /// Ok(()) on success, LoggerError on failure
    ///
  
    pub fn write_message(&self, message: &str, file_path: &Path) -> LoggerResult<()> {
        // Step 1: Ensure directory exists
        self.ensure_directory_exists(file_path)?;

        // Step 2: Open file in append mode (create if doesn't exist)
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .map_err(|_| LoggerError::FileCreationFailed {
                path: file_path.display().to_string(),
                reason: "Failed to open file for writing".to_string(),
            })?;

        // Step 3: Use buffered writer for better performance
        let mut writer = BufWriter::new(file);

        // Step 4: Write message with newline
        writeln!(writer, "{}", message)
            .map_err(|_| LoggerError::DiskFull {
                path: file_path.display().to_string(),
                bytes_attempted: message.len() + 1, // +1 for newline
            })?;

        // Step 5: Flush to ensure data is written to disk
        writer.flush()
            .map_err(|_| LoggerError::DiskFull {
                path: file_path.display().to_string(),
                bytes_attempted: message.len() + 1,
            })?;

        Ok(())
    }

    /// Ensure the directory for the log file exists
    ///
    /// Creates parent directories recursively if they don't exist.
    /// This is called automatically by write_message.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file (directory will be extracted)
    ///
    /// # Returns
    /// Ok(()) if directory exists or was created successfully
    fn ensure_directory_exists(&self, file_path: &Path) -> LoggerResult<()> {
        if let Some(parent_dir) = file_path.parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)
                    .map_err(|_| LoggerError::DirectoryCreationFailed {
                        path: parent_dir.display().to_string(),
                        reason: "Failed to create parent directories".to_string(),
                    })?;
            }
        }
        Ok(())
    }

    /// Check if we can write to the specified file path
    ///
    /// Tests write permissions without actually writing log data.
    /// Useful for validation during logger initialization.
    ///
    /// # Arguments
    /// * `file_path` - Path to test
    ///
    /// # Returns
    /// Ok(()) if writable, LoggerError if not
    pub fn test_write_permissions(&self, file_path: &Path) -> LoggerResult<()> {
        // Try to write an empty test message
        self.write_message("", file_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_write_message_creates_file() {
        let temp_dir = tempdir().unwrap();
        let log_path = temp_dir.path().join("test.log");

        let writer = TextWriter::new();
        let result = writer.write_message("Test log message", &log_path);

        assert!(result.is_ok());
        assert!(log_path.exists());

        let content = fs::read_to_string(&log_path).unwrap();
        assert_eq!(content, "Test log message\n");
    }

    #[test]
    fn test_write_message_appends_to_existing_file() {
        let temp_dir = tempdir().unwrap();
        let log_path = temp_dir.path().join("test.log");

        let writer = TextWriter::new();

        // Write first message
        writer.write_message("First message", &log_path).unwrap();

        // Write second message
        writer.write_message("Second message", &log_path).unwrap();

        let content = fs::read_to_string(&log_path).unwrap();
        assert_eq!(content, "First message\nSecond message\n");
    }

    #[test]
    fn test_ensure_directory_creates_nested_dirs() {
        let temp_dir = tempdir().unwrap();
        let log_path = temp_dir.path().join("logs").join("app").join("test.log");

        let writer = TextWriter::new();
        let result = writer.write_message("Test", &log_path);

        assert!(result.is_ok());
        assert!(log_path.exists());
        assert!(log_path.parent().unwrap().exists());
    }

    #[test]
    fn test_empty_message_handling() {
        let temp_dir = tempdir().unwrap();
        let log_path = temp_dir.path().join("empty.log");

        let writer = TextWriter::new();
        let result = writer.write_message("", &log_path);

        assert!(result.is_ok());

        let content = fs::read_to_string(&log_path).unwrap();
        assert_eq!(content, "\n"); // Just a newline
    }
}