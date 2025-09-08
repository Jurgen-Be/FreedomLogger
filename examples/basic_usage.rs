use FreedomLogger::{log_init_with_level, log_info, log_warning, log_error, Pattern, LogLevel, Logger};

fn main() {
    // Detailed pattern met file:line info
    FreedomLogger::log_init_with_level(
        Pattern::Detailed,
        "./logs",
        "detailed_test",
        LogLevel::Debug
    );

    log_info("This will show file and line number");
    log_warning("Warning from main function");
    log_error("Error with location info");

    println!("Check ./logs/detailed_test.log");
}