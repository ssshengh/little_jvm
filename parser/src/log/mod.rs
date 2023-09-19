#[cfg(test)]
mod test;
use std::io::Write;
use env_logger::Target;
use log::{debug, info};

pub enum LogLevel {
    INFO,
    WARN,
    DEBUG,
    ERROR,
    TRACE,
}


/// 初始化 log
/// 可以参考 https://llever.com/rust-cookbook-zh/development_tools/debugging/config_log.zh.html
pub fn init_log(level: LogLevel) {
    update_log_level(level);
    let mut builder = env_logger::builder();

    // 可以自定义格式
    // builder.format(|buf, record| {
    //     writeln!(buf, "{}: {}", record.level(), record.args())
    // });

    builder.target(Target::Stdout);
    builder.init();
    debug!("Logger has init as info level");
}

fn update_log_level(level: LogLevel) {
    match level {
        LogLevel::INFO => std::env::set_var("RUST_LOG", "info"),
        LogLevel::WARN => std::env::set_var("RUST_LOG", "warn"),
        LogLevel::DEBUG => std::env::set_var("RUST_LOG", "debug"),
        LogLevel::ERROR => std::env::set_var("RUST_LOG", "error"),
        LogLevel::TRACE => std::env::set_var("RUST_LOG", "trace")
    }
}

