# FreedomLogger

A professional, thread-safe logging library for Rust with automatic rotation, multiple output formats, and error-proof operation.

## Features

- **Multiple log levels** with filtering (ERROR, WARNING, INFO, DEBUG, TRACE)
- **Various output patterns** from basic to JSON structured logging
- **Automatic log rotation** based on configurable file size limits
- **Thread-safe concurrent logging** with internal synchronization
- **Error-proof operation** - internal errors never crash your application
- **Minimal dependencies** - only chrono for timestamps
- **Easy initialization** - single function call setup

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
freedom_logger = { git = "https://github.com/Jurgen-Be/FreedomLogger" }
```

Basic usage:
```rust
use freedom_logger::{init, info, warning, error, Pattern};

fn main() {
    // Initialize logger once
    FreedomLogger::init(Pattern::Basic, "./logs", "myapp");
    
    // Log anywhere in your application
    info("Application started");
    warning("This is a warning");
    error("Something went wrong");
}
```

## Installation

```bash
cargo add FreedomLogger
```

## Usage Examples

### Basic Initialization
```rust
use FreedomLogger::{init, Pattern};

// Logs all levels, 10MB files, 5 backups
init(Pattern::Basic, "/var/log/myapp", "application");
```

### With Log Level Filtering
```rust
use FreedomLogger::{init_with_level, Pattern, LogLevel};

// Only log WARNING and ERROR messages
init_with_level(Pattern::Detailed, "./logs", "app", LogLevel::Warning);
```

### Custom Rotation Settings
```rust
use FreedomLogger::{init_with_rotation, Pattern, LogLevel};

// 50MB files, keep 10 backups
init_with_rotation(
    Pattern::Json,
    "./logs", 
    "service",
    LogLevel::Info,
    50 * 1024 * 1024,  // 50MB
    10                 // 10 backup files
);
```

### Logging Functions
```rust
use FreedomLogger::{error, warning, info, debug, trace};

error("Critical system failure");
warning("Deprecated API usage detected");  
info("User authentication successful");
debug("Processing request payload");
trace("Entering function calculate_metrics");
```

## Output Patterns

### Basic Pattern
```
[2025-09-08 14:30:45] INFO: User logged in successfully
[2025-09-08 14:30:46] ERROR: Database connection failed
```

### Detailed Pattern
```
[2025-09-08 14:30:45] [main.rs:42] INFO: User logged in successfully
[2025-09-08 14:30:46] [db.rs:158] ERROR: Database connection failed
```

### JSON Pattern
```json
{"timestamp":"2025-09-08 14:30:45","level":"INFO","message":"User logged in successfully","file":"main.rs","line":42,"thread":"main"}
{"timestamp":"2025-09-08 14:30:46","level":"ERROR","message":"Database connection failed","file":"db.rs","line":158,"thread":"worker-1"}
```

## Log Rotation

FreedomLogger automatically rotates log files when they exceed the configured size:

- `app.log` (current log file)
- `app.1.log` (most recent backup)
- `app.2.log` (older backup)
- `app.N.log` (oldest backup, deleted when limit reached)

Default settings: 10MB max file size, 5 backup files retained.

## Error Handling

FreedomLogger is designed to be error-proof:

- **Never panics** - Internal errors are handled gracefully
- **Silent operation** - Logging failures don't interrupt your application
- **Separate error log** - Internal issues logged to `logger_errors.log`
- **Automatic fallbacks** - Invalid configurations use safe defaults
- **Directory creation** - Creates log directories automatically

## Thread Safety

FreedomLogger is fully thread-safe:

```rust
use std::thread;
use FreedomLogger::{init, info, Pattern};

fn main() {
    init(Pattern::Basic, "./logs", "threaded_app");
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                info(&format!("Message from thread {}", i));
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## Configuration Options

### Initialization Functions

| Function | Log Level | Rotation | Use Case |
|----------|-----------|----------|----------|
| `init()` | All levels | Default (10MB, 5 backups) | Development, testing |
| `init_with_level()` | Filtered | Default (10MB, 5 backups) | Production with filtering |
| `init_with_rotation()` | Filtered | Custom | High-volume production |

### Log Levels (Hierarchical)

- **ERROR** - Critical failures, system errors
- **WARNING** - Potential issues, deprecated usage
- **INFO** - General application flow information
- **DEBUG** - Detailed debugging information
- **TRACE** - Very verbose tracing information

### Patterns

- **Basic** - Simple timestamp, level, message format
- **Detailed** - Includes source file and line number
- **Extended** - Adds thread information (planned)
- **JSON** - Structured logging for analysis tools
- **Custom** - User-defined format strings (planned)

## File Extensions

FreedomLogger automatically uses appropriate file extensions:

- **Text patterns** (Basic, Detailed, Extended, Custom) → `.log` files
- **JSON pattern** → `.json` files

## Performance

- **Buffered I/O** - Uses `BufWriter` for optimal write performance
- **Minimal allocations** - Efficient string formatting and memory usage
- **Thread synchronization** - Mutex-protected writes prevent data corruption
- **Lazy initialization** - Logger components created only when needed

## Examples

Complete examples are available in the `examples/` directory:

```bash
# Basic logging example
cargo run --example basic_usage

# JSON structured logging
cargo run --example json_logging

# High-volume logging with rotation
cargo run --example rotation_demo
```

## Requirements

- **Rust** 1.70 or later
- **Dependencies**: chrono (timestamps), tempfile (dev/testing only)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Roadmap

### Version 2.0 (Planned)
- Database integration for direct log storage
- Time-based rotation (daily, weekly, monthly)
- Async logging for high-performance applications
- Enhanced caller location tracking
- Full custom pattern parsing

### Version 1.x (Maintenance)
- Bug fixes and performance improvements
- Additional output formats
- Extended platform support

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for detailed version history.

## Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/FreedomLogger/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/FreedomLogger/discussions)
- **Documentation**: [docs.rs/FreedomLogger](https://docs.rs/FreedomLogger)

---

*Built with Rust for performance, safety, and reliability.*