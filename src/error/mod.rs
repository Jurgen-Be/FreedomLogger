/// File: src/error/mod.rs

/// This module contains all error types and error handling logic for internal
/// logger failures. When the logger itself encounters problems (disk full,
/// permission denied, etc.), these errors are caught and handled silently.
///
/// The user's application never sees these errors - they are logged to a
/// separate error file for debugging purposes only.

use std::fs::{File, OpenOptions};
use std::io::{Write, BufWriter};
use std::path::Path;

// Re-export all error types for easy importing
pub use types::*;

mod types;

// Writes internal logger errors to the error log file
///
/// This function handles errors that occur within the FreedomLogger itself.
/// It writes to 'logger_errors.log' in the same directory as the main log file.
/// If this function itself fails (e.g., can't create an error log), it silently fails.
///
/// # Arguments
/// * `error` - The LoggerError to write to the error log
/// * `log_directory` - Directory where the main log file is located

pub fn write_error_to_log(error: &LoggerError, log_directory: &Path) {
    // Create an error log path in the same directory as the main log
    let error_log_path = log_directory.join("logger_errors.log");

    /*
    Try to write error - if this fails, we silently give up
    We cannot recursively handle error from error handling
     */
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(error_log_path)
    {
        let timestamp = get_current_timestamp();
        let error_message = format!("[{}] FreedomLogger Error: {}\n", timestamp, error);

        // Use BufWriter for better performance when writing errors
        let mut writer = BufWriter::new(&mut file);
        let _ = writer.write_all(error_message.as_bytes());
        let _ = writer.flush();
    }

    // If we can't write to the error log, we silently fail.
    // This prevents infinite error loops
}
fn get_current_timestamp() -> String {
    use chrono::{Local, DateTime};
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}