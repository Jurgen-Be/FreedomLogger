use FreedomLogger::{init_with_level, info, warning, error, Pattern, LogLevel};

fn main() {
    // Detailed pattern met file:line info
    FreedomLogger::init_with_level(
        Pattern::Detailed,
        "./logs",
        "detailed_test",
        LogLevel::Debug
    );

    info("This will show file and line number");
    warning("Warning from main function");
    error("Error with location info");

    println!("Check ./logs/detailed_test.log");
}