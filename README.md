# FreedomLogger 🦀

A professional, thread-safe logging library for Rust with automatic rotation, multiple output formats, and error-proof operation.

## ✨ Features

- **Multiple log levels** with filtering (ERROR, WARNING, INFO, DEBUG, TRACE)
- **Flexible logging API** - Both simple functions and formatted macros
- **Various output patterns** from basic to JSON structured logging
- **Automatic log rotation** based on configurable file size limits
- **Thread-safe concurrent logging** with internal synchronization
- **Error-proof operation** - internal errors never crash your application
- **Minimal dependencies** - only chrono for timestamps
- **Easy initialization** - single function call setup

## 🚀 Quick Start

### Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
freedom_logger = "1.1.0"
```

Or use cargo:
```bash
cargo add freedom_logger
```

### Basic usage:
```rust
use freedom_logger::{log_init, log_info, log_warning, log_error, Pattern};

fn main() {
    // Initialize logger once
    log_init(Pattern::Basic, "./logs", "myapp");
    
    // Log anywhere in your application
    log_info("Application started");
    log_warning("This is a warning");
    log_error("Something went wrong");
}
```

## 🆕 New in v1.1.0: Formatted Logging Macros

FreedomLogger now supports both simple functions and powerful formatting macros:

### Simple Functions (Original)
```rust
use freedom_logger::{log_error, log_warning, log_info, log_debug, log_trace};

log_error("Critical system failure");
log_warning("Deprecated API usage detected");
log_info("User authentication successful");
log_debug("Processing request payload");
log_trace("Entering function calculate_metrics");
```

### New Formatting Macros
```rust
use freedom_logger::{log_error, log_warning, log_info, log_debug, log_trace};

// Support for formatted strings with automatic type handling
log_info!("User {} logged in successfully", username);
log_debug!("Database path: {:?}", database_path);  // Works with any Debug type!
log_error!("Failed to connect to {}: {}", host, error_message);
log_warning!("Processing {} items in batch {}", item_count, batch_id);

// Complex types work automatically
let config = MyConfig { host: "localhost", port: 5432 };
log_debug!("Server config: {:?}", config);

// Multiple format specifiers
log_info!("User {} from {} logged in at {}", user_id, ip_address, timestamp);
```

### Why Use the Macros?

**Before v1.1.0** (would cause compilation errors):
```rust
let database_path = PathBuf::from("/var/lib/app.db");
log_debug("Database path: {:?}", database_path); // ❌ Error!
```

**After v1.1.0** (works perfectly):
```rust
let database_path = PathBuf::from("/var/lib/app.db");
log_debug!("Database path: {:?}", database_path); // ✅ Perfect!
```

## 📋 Initialization Options

### Basic Setup
```rust
use freedom_logger::{log_init, Pattern};

// Logs all levels, 10MB files, 5 backups
log_init(Pattern::Basic, "/var/log/myapp", "application");
```

### With Log Level Filtering
```rust
use freedom_logger::{log_init_with_level, Pattern, LogLevel};

// Only log WARNING and ERROR messages
log_init_with_level(Pattern::Detailed, "./logs", "app", LogLevel::Warning);
```

### Full Configuration
```rust
use freedom_logger::{log_init_with_rotation, Pattern, LogLevel};

// 50MB files, keep 10 backups
log_init_with_rotation(
    Pattern::Json,
    "./logs",
    "service",
    LogLevel::Info,
    50 * 1024 * 1024, // 50MB
    10 // 10 backup files
);
```

## 📝 Output Formats

### Basic Pattern
```
[2025-09-09 14:30:45] INFO: User logged in successfully
[2025-09-09 14:30:46] ERROR: Database connection failed
```

### Detailed Pattern (with source location)
```
[2025-09-09 14:30:45] [main.rs:42] INFO: User logged in successfully
[2025-09-09 14:30:46] [db.rs:158] ERROR: Database connection failed
```

### JSON Pattern (structured logging)
```json
{"timestamp":"2025-09-09 14:30:45","level":"INFO","message":"User logged in successfully","file":"main.rs","line":42,"thread":"main"}
{"timestamp":"2025-09-09 14:30:46","level":"ERROR","message":"Database connection failed","file":"db.rs","line":158,"thread":"worker-1"}
```

## 🔄 Automatic Log Rotation

FreedomLogger automatically rotates log files when they exceed the configured size:

```
app.log      (current log file)
app.1.log    (most recent backup)
app.2.log    (older backup)
app.N.log    (oldest backup, deleted when limit reached)
```

**Default settings:** 10MB max file size, 5 backup files retained.

## 🛡️ Error-Proof Operation

FreedomLogger is designed to be error-proof:

- **Never panics** - Internal errors are handled gracefully
- **Silent operation** - Logging failures don't interrupt your application
- **Separate error log** - Internal issues logged to `logger_errors.log`
- **Automatic fallbacks** - Invalid configurations use safe defaults
- **Directory creation** - Creates log directories automatically

## 🧵 Thread Safety

FreedomLogger is fully thread-safe:

```rust
use std::thread;
use freedom_logger::{log_init, Pattern};

fn main() {
    log_init(Pattern::Basic, "./logs", "threaded_app");
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                // Both styles work in threads
                log_info!("Message from thread {}", i);
                log_debug!("Thread {} processing data: {:?}", i, some_data);
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## 📊 Configuration Quick Reference

| Function | Log Level | Rotation | Use Case |
|----------|-----------|----------|----------|
| `log_init()` | All levels | Default (10MB, 5 backups) | Development, testing |
| `log_init_with_level()` | Filtered | Default (10MB, 5 backups) | Production with filtering |
| `log_init_with_rotation()` | Filtered | Custom | High-volume production |

## 📈 Log Levels

- **ERROR** - Critical failures, system errors
- **WARNING** - Potential issues, deprecated usage
- **INFO** - General application flow information
- **DEBUG** - Detailed debugging information
- **TRACE** - Very verbose tracing information

## 🎨 Available Patterns

- **Basic** - Simple timestamp, level, message format
- **Detailed** - Includes source file and line number
- **Extended** - Adds thread information (planned)
- **JSON** - Structured logging for analysis tools
- **Custom** - User-defined format strings (planned)

## 📁 File Extensions

FreedomLogger automatically uses appropriate file extensions:
- Text patterns (Basic, Detailed, Extended, Custom) → `.log` files
- JSON pattern → `.json` files

## ⚡ Performance

- **Buffered I/O** - Uses `BufWriter` for optimal write performance
- **Minimal allocations** - Efficient string formatting and memory usage
- **Thread synchronization** - Mutex-protected writes prevent data corruption
- **Lazy initialization** - Logger components created only when needed

## 📚 Examples

Complete examples are available in the `examples/` directory:

```bash
# Basic logging example
cargo run --example basic_usage

# JSON structured logging  
cargo run --example json_logging

# High-volume logging with rotation
cargo run --example rotation_demo

# NEW: Formatted logging examples
cargo run --example formatted_logging
```

## 📋 Requirements

- Rust 1.70 or later
- Dependencies: chrono (timestamps), tempfile (dev/testing only)

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 🛣️ Roadmap

### v2.0.0 (Planned)
- Database integration for direct log storage
- Time-based rotation (daily, weekly, monthly)
- Async logging for high-performance applications
- Enhanced caller location tracking
- Full custom pattern parsing

### Ongoing
- Bug fixes and performance improvements
- Additional output formats
- Extended platform support

## 📄 License

Licensed under the **MIT License**.

See [CHANGELOG](CHANGELOG.md) for detailed version history.

## 🔗 Links

- **Issues**: [GitHub Issues](https://github.com/Jurgen-Be/FreedomLogger/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Jurgen-Be/FreedomLogger/discussions)
- **Documentation**: [docs.rs/freedom_logger](https://docs.rs/freedom_logger)

---

**Built with Rust for performance, safety, and reliability.** 🦀