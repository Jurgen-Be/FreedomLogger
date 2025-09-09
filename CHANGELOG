# Changelog

All notable changes to FreedomLogger will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-09-09

### Added

#### Enhanced Logging API
- **Formatted logging macros**: Added `log_error!()`, `log_warning!()`, `log_info!()`, `log_debug!()`, and `log_trace!()` macros
- **Format string support**: Macros support Rust's standard format strings with automatic type handling
- **Flexible type support**: Automatically handles types implementing `Display` or `Debug` traits
- **Backward compatibility**: Original functions (`log_info("message")`) continue to work unchanged

#### New Macro Features
- **Debug formatting**: Support for `{:?}` formatting for complex types like `PathBuf`, structs, etc.
- **Multiple arguments**: Support for multiple format arguments: `log_info!("User {} logged in at {}", user_id, timestamp)`
- **Automatic type conversion**: Uses Rust's built-in `format!` macro for robust type handling
- **Trailing comma support**: Optional trailing commas in macro arguments for better code formatting

#### Usage Examples
```rust
// Original functions (still work)
log_info("Application started");
log_debug("Simple debug message");

// New macros with formatting
log_info!("User {} logged in", username);
log_debug!("Database path: {:?}", database_path);
log_error!("Failed to connect to {}: {}", host, error);
log_warning!("Processing {} items in batch {}", count, batch_id);
```

### Technical Implementation

#### Macro Design
- **Dual syntax support**: Each macro supports both simple messages and formatted strings
- **Zero runtime overhead**: Macros expand to existing function calls with pre-formatted strings
- **Error-proof compilation**: Leverages Rust's compile-time format string validation
- **Consistent API**: All log levels have identical macro interfaces

#### Testing
- **Enhanced test coverage**: Updated integration tests to verify both function and macro approaches
- **Format validation**: Tests confirm proper handling of various data types and format strings
- **Backward compatibility testing**: Ensures existing code continues to work without changes

### Fixed
- **Global logger initialization**: Improved test reliability by handling single global logger instance correctly
- **Test organization**: Consolidated tests to prevent conflicts from multiple logger initialization attempts

### Developer Experience
- **No breaking changes**: All existing code continues to work without modification
- **Progressive enhancement**: Users can gradually adopt new macro syntax where beneficial
- **Type flexibility**: Solves common issues where users couldn't log complex types without manual formatting

---

## [1.0.0] - 2025-09-08

### Added

#### Core Features
- **Multiple log levels**: ERROR, WARNING, INFO, DEBUG, TRACE with hierarchical filtering
- **Thread-safe logging**: Concurrent access support with internal mutex synchronization
- **Simple initialization API**: Single-call setup with `logger::init()` family functions
- **Global logger instance**: Access logging functions anywhere in codebase after initialization

#### Log Patterns
- **Basic pattern**: `[TIMESTAMP] LEVEL: MESSAGE`
- **Detailed pattern**: `[TIMESTAMP] [FILE:LINE] LEVEL: MESSAGE` with source location
- **Extended pattern**: `[TIMESTAMP] [FILE:LINE] [THREAD] LEVEL: MESSAGE` (fallback to detailed)
- **JSON pattern**: Structured logging with consistent field schema
- **Custom pattern**: User-defined format strings (fallback to basic)

#### Output Formats
- **Plain text logging**: Human-readable `.log` files
- **JSON logging**: Structured `.json` files (JSONL format, one object per line)
- **Automatic file extension**: `.log` for text patterns, `.json` for JSON pattern

#### Log Rotation
- **Size-based rotation**: Automatic rotation when files exceed configured size limit
- **Configurable backup retention**: Keep specified number of backup files (`.1.log`, `.2.log`, etc.)
- **Rolling backup scheme**: Older files get higher numbers, oldest files are deleted
- **Default settings**: 10MB max file size, 5 backup files retained

#### Initialization Options
- `logger::init(pattern, path, filename)` - Basic setup, logs all levels
- `logger::init_with_level(pattern, path, filename, level)` - With level filtering
- `logger::init_with_rotation(pattern, path, filename, level, max_size, max_backups)` - Full control

#### Error Handling
- **Error-proof operation**: Logger never panics or crashes user applications
- **Silent error handling**: Internal logger errors don't propagate to user code
- **Separate error logging**: Internal errors logged to `logger_errors.log` for debugging
- **Graceful degradation**: Invalid patterns fall back to basic pattern
- **Automatic directory creation**: Creates log directories if they don't exist

#### Dependencies
- **Minimal external dependencies**: Only chrono for timestamp formatting
- **No runtime dependencies**: Self-contained logging without heavy external crates

### Technical Implementation

#### Architecture
- **Modular design**: Separate modules for config, formatting, rotation, writing, and errors
- **Professional code structure**: Organized in logical directories with clear responsibilities
- **Comprehensive error types**: Specific error variants for different failure scenarios
- **Builder pattern configuration**: Clean API for logger setup with sensible defaults

#### Testing
- **19 comprehensive tests**: Unit tests covering all major functionality
- **Integration testing**: End-to-end logging workflow validation
- **Rotation testing**: File rotation logic with temporary directories
- **Format testing**: All log patterns with various input scenarios
- **Writer testing**: Both text and JSON output validation

#### Thread Safety
- **Mutex-protected writes**: Thread-safe file operations
- **Global singleton pattern**: Single logger instance accessible across threads
- **Concurrent logging support**: Multiple threads can log simultaneously without data corruption

### Known Limitations

- **File/line information**: Currently shows internal logger location instead of caller location
  - Will be addressed in future version with macro-based approach or `#[track_caller]`
- **Time-based rotation**: Only size-based rotation currently implemented
  - Daily/weekly/monthly rotation planned for future versions
- **Database logging**: JSON format ready for database integration in v2.0
- **Custom pattern parsing**: Full custom pattern implementation planned for future release

### Documentation

- **Comprehensive inline documentation**: All public functions and types documented
- **English docstrings**: Consistent documentation language
- **README files**: Both English and Dutch documentation planned
- **Usage examples**: Complete examples for all initialization patterns

---

## Planned for v2.0.0

### Database Integration
- Direct database logging support through JSON writer extension
- Multiple database backends (SQLite, PostgreSQL, MySQL)
- Structured query interface for log analysis

### Enhanced Patterns
- Full custom pattern parsing with placeholder support
- Time-based rotation (daily, weekly, monthly)
- Caller location tracking for accurate file/line information

### Performance Improvements
- Asynchronous logging option for high-throughput applications
- Configurable buffer sizes for batch writing
- Memory-mapped file support for large log files

---

*For more information, visit the [FreedomLogger repository](https://github.com/Jurgen-Be/FreedomLogger)*