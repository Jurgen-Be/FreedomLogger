// File: src/error/types.rs

/*
Error types for FreedomLogger internal operations.
These errors represents failures that can occur within the logger itself.
When these errors occur, they are silently handled and optionally.
Written to a separate error log fie for debugging.
 */
use std::fmt;

/// Represents all possible internal failures of the FreedomLogger.
#[derive(Debug, Clone, PartialEq)]
pub enum LoggerError {
    /*
    Failed to create the log file
    Occurs when: file doesn't exist and creation fails
     */
    FileCreationFailed {
        path: String,
        reason: String,
    },

    /*
    Failed to create the necessary directories
    Occurs when: directory doesn't exist and creation fails
     */
    DirectoryCreationFailed {
        path: String,
        reason: String,
    },

    /*
    Not enough permissions to write to log file
    Occurs when: file exists but can't write to it
     */
    WritePermissionDenied {
        path: String,
    },

    /*
    Disk space insufficient for writing
    Occurs when: write operation fails due disk full
     */
    DiskFull {
        path: String,
        bytes_attempted: usize,
    },

    /*
    Log rotation operation failed
    Occurs when: log rotation fails
     */
    RotationFailed {
        current_file: String,
        backup_file: String,
        reason: String,
    },
}

impl fmt::Display for LoggerError {
    /*
    Human-readable error messages for debugging
    These will be written to the error log file.
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoggerError::FileCreationFailed {path, reason} => {
                write!(f, "Failed to create log file '{}' : {}", path, reason)
            }
            LoggerError::DirectoryCreationFailed {path,reason} => {
                write!(f, "Failed to create directory '{}': {}", path, reason)
            }

            LoggerError::WritePermissionDenied {path} => {
                write!(f, "Persmission denied writing to '{}'", path)
            }

            LoggerError::DiskFull {path, bytes_attempted} => {
                write!(f, "Disk full: could not write {} bytes to '{}'", bytes_attempted, path)
            }

            LoggerError::RotationFailed {current_file, backup_file, reason} => {
                write!(f, "Log rotation failed: '{}' -> '{}': {}", current_file, backup_file, reason)
            }
        }
    }
}

impl std::error::Error for LoggerError {}

/*
Result type for logging operations
Used internally - user never sees this result
 */
pub type LoggerResult<T> = Result<T, LoggerError>;