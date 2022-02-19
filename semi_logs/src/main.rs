#[derive(Debug)]
enum LogLevel {
    Info,
    Warning,
    Error
}

fn log(log_level: LogLevel, message: &str) -> String {
    format!("[{log_level:?}]: {message}")
}

fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}



fn main() {
    println!("{}", log(LogLevel::Error, "Stack Overflow"));
    println!("{}", log(LogLevel::Warning, "Deprecated"));
    println!("{}", info("Timezone changed"));
}
