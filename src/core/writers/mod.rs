/// Writers module for FreedomLogger
///
/// This module contains all log writers responsible for outputting formatted
/// log data to different destinations and formats:
///
/// - TextWriter: Plain text files (.log extension)
/// - JsonWriter: Structured JSON files (.json extension)
///
/// Future v2 enhancements will extend JsonWriter to support database output
/// while maintaining the same interface.

// Re-export all writer types
pub use text::TextWriter;
pub use json::JsonWriter;

// Import writer implementations
pub mod text;
pub mod json;