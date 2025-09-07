/// Log rotation module for FreedomLogger
///
/// This module handles automatic log file rotation to prevent files from
/// growing too large. Currently supports size-based rotation, with future
/// plans for time-based rotation (daily, weekly, monthly).
///
/// Rotation strategies:
/// - Size-based: Rotate when file exceeds configured size limit
/// - Time-based: Rotate at specific time intervals (TODO: future feature)

// Re-export all rotation types and functions
pub use size_based::{SizeBasedRotation, RotationResult};

// Import rotation implementations  
pub mod size_based;

// TODO: Future rotation strategies
// pub mod time_based;  // Daily, weekly, monthly rotation