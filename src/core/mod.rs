/// Core module for FreedomLogger
///
/// Contains the main logger implementation and all supporting components:
/// - Logger: Main logging orchestrator
/// - LoggerConfig: Configuration management
/// - Writers: Text and JSON output handlers
/// - Configuration types: LogLevel, Pattern, etc.

// Re-export the main Logger struct
pub use logger::Logger;

// Re-export configuration types for public API
pub use config::{LogLevel, Pattern, LoggerConfig};

// Re-export writers for potential advanced usage
pub use writers::{TextWriter, JsonWriter};

// Import all core modules
pub mod config;
pub mod logger;
pub mod writers;