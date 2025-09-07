// File: src/format/mod.rs


/// Format module for FreedomLogger
///
/// This module contains all log formatting patterns and utilities.
/// Each pattern (Basic, Detailed, Extended, Json, Custom) has its own
/// formatter function that converts LogInfo into formatted strings.
///
/// The formatters are organized from simple to complex:
/// - Basic: Just timestamp, level, message
/// - Detailed: Adds file and line information
/// - Extended: Adds thread information (TODO)
/// - Json: Structured JSON output (TODO)
/// - Custom: User-defined patterns (TODO)

// Re-export LogInfo struct for other modules to use
pub use basic::LogInfo;

// Re-export all formatter functions
pub use basic::format_basic;
pub use detailed::format_detailed;

// Import the formatter functions
pub mod basic;
pub mod detailed;


// TODO: Future formatters to implement
// pub mod extended;
// pub mod json;
// pub mod custom;