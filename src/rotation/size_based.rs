/// Size-based log rotation for FreedomLogger
///
/// Handles automatic log file rotation when files exceed the configured size limit.
/// Maintains a configurable number of backup files in rolling fashion:
/// - app.log (current)
/// - app.1.log (most recent backup)
/// - app.2.log (older backup)
/// - ...
/// - app.N.log (oldest backup, gets deleted when limit reached)

use std::fs;
use std::path::{Path, PathBuf};
use crate::error::{LoggerError, LoggerResult};

/// Represents the result of a rotation check
#[derive(Debug, PartialEq)]
pub enum RotationResult {
    /// No rotation needed - file is still below size limit
    NotNeeded,
    /// Rotation completed successfully
    Completed,
    /// Rotation was needed but failed
    Failed(LoggerError),
}

/// Size-based rotation manager
///
/// Handles checking file sizes and performing rotation when necessary.
/// Uses a rolling backup scheme where older files get higher numbers.
#[derive(Debug)]
pub struct SizeBasedRotation {
    /// Maximum file size in bytes before rotation
    max_file_size: u64,
    /// Maximum number of backup files to keep
    max_backup_files: u32,
}

impl SizeBasedRotation {
    /// Create a new size-based rotation manager
    ///
    /// # Arguments
    /// * `max_file_size` - Maximum size in bytes before rotation (e.g., 10MB = 10 * 1024 * 1024)
    /// * `max_backup_files` - Number of backup files to keep (e.g., 5 keeps .1 through .5)
    pub fn new(max_file_size: u64, max_backup_files: u32) -> Self {
        Self {
            max_file_size,
            max_backup_files,
        }
    }

    /// Check if rotation is needed and perform it if necessary
    ///
    /// # Arguments
    /// * `log_file_path` - Path to the current log file
    ///
    /// # Returns
    /// RotationResult indicating what happened
    pub fn check_and_rotate(&self, log_file_path: &Path) -> RotationResult {
        match self.needs_rotation(log_file_path) {
            Ok(true) => self.perform_rotation(log_file_path),
            Ok(false) => RotationResult::NotNeeded,
            Err(error) => RotationResult::Failed(error),
        }
    }

    /// Check if the log file needs rotation based on size
    ///
    /// # Arguments
    /// * `log_file_path` - Path to check
    ///
    /// # Returns
    /// Ok(true) if rotation needed, Ok(false) if not, Err if can't check
    fn needs_rotation(&self, log_file_path: &Path) -> LoggerResult<bool> {
        match fs::metadata(log_file_path) {
            Ok(metadata) => Ok(metadata.len() >= self.max_file_size),
            Err(_) => {
                // File doesn't exist yet - no rotation needed
                Ok(false)
            }
        }
    }

    /// Perform the actual rotation process
    ///
    /// Steps:
    /// 1. Delete oldest backup file if it exists
    /// 2. Shift all backup files up one number (app.1.log → app.2.log)
    /// 3. Move current file to .1 backup (app.log → app.1.log)
    /// 4. Current log file slot is now empty for new logs
    fn perform_rotation(&self, log_file_path: &Path) -> RotationResult {
        let base_name = match log_file_path.file_stem() {
            Some(name) => name.to_string_lossy(),
            None => return RotationResult::Failed(LoggerError::RotationFailed {
                current_file: log_file_path.display().to_string(),
                backup_file: "unknown".to_string(),
                reason: "Invalid file path".to_string(),
            }),
        };

        let directory = log_file_path.parent().unwrap_or(Path::new("."));

        // Step 1: Delete oldest backup if it exists
        if self.max_backup_files > 0 {
            let oldest_backup = directory.join(format!("{}.{}.log", base_name, self.max_backup_files));
            if oldest_backup.exists() {
                if let Err(_) = fs::remove_file(&oldest_backup) {
                    return RotationResult::Failed(LoggerError::RotationFailed {
                        current_file: log_file_path.display().to_string(),
                        backup_file: oldest_backup.display().to_string(),
                        reason: "Failed to delete oldest backup".to_string(),
                    });
                }
            }
        }

        // Step 2: Shift existing backups up one number (reverse order to avoid conflicts)
        for i in (1..self.max_backup_files).rev() {
            let current_backup = directory.join(format!("{}.{}.log", base_name, i));
            let next_backup = directory.join(format!("{}.{}.log", base_name, i + 1));

            if current_backup.exists() {
                if let Err(_) = fs::rename(&current_backup, &next_backup) {
                    return RotationResult::Failed(LoggerError::RotationFailed {
                        current_file: current_backup.display().to_string(),
                        backup_file: next_backup.display().to_string(),
                        reason: "Failed to shift backup file".to_string(),
                    });
                }
            }
        }

        // Step 3: Move current log to first backup position
        if self.max_backup_files > 0 {
            let first_backup = directory.join(format!("{}.1.log", base_name));
            if let Err(_) = fs::rename(log_file_path, &first_backup) {
                return RotationResult::Failed(LoggerError::RotationFailed {
                    current_file: log_file_path.display().to_string(),
                    backup_file: first_backup.display().to_string(),
                    reason: "Failed to move current log to backup".to_string(),
                });
            }
        } else {
            // No backups configured - just delete current file
            if let Err(_) = fs::remove_file(log_file_path) {
                return RotationResult::Failed(LoggerError::RotationFailed {
                    current_file: log_file_path.display().to_string(),
                    backup_file: "none".to_string(),
                    reason: "Failed to delete current log (no backups configured)".to_string(),
                });
            }
        }

        RotationResult::Completed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_rotation_not_needed_for_small_file() {
        let temp_dir = tempdir().unwrap();
        let log_path = temp_dir.path().join("test.log");

        // Create small file (100 bytes)
        let mut file = File::create(&log_path).unwrap();
        file.write_all(&vec![b'x'; 100]).unwrap();

        let rotation = SizeBasedRotation::new(1000, 3); // 1KB limit
        let result = rotation.check_and_rotate(&log_path);

        assert_eq!(result, RotationResult::NotNeeded);
    }

    #[test]
    fn test_rotation_needed_for_large_file() {
        let temp_dir = tempdir().unwrap();
        let log_path = temp_dir.path().join("test.log");

        // Create large file (2KB)
        let mut file = File::create(&log_path).unwrap();
        file.write_all(&vec![b'x'; 2048]).unwrap();
        drop(file); // Close file

        let rotation = SizeBasedRotation::new(1000, 2); // 1KB limit, 2 backups
        let result = rotation.check_and_rotate(&log_path);

        assert_eq!(result, RotationResult::Completed);

        // Original file should be gone (rotated to backup)
        assert!(!log_path.exists());

        // Backup should exist
        let backup_path = temp_dir.path().join("test.1.log");
        assert!(backup_path.exists());
    }

    #[test]
    fn test_no_rotation_for_nonexistent_file() {
        let temp_dir = tempdir().unwrap();
        let log_path = temp_dir.path().join("nonexistent.log");

        let rotation = SizeBasedRotation::new(1000, 3);
        let result = rotation.check_and_rotate(&log_path);

        assert_eq!(result, RotationResult::NotNeeded);
    }
}