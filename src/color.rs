use log;
use log::LogLevel;
use colored::*;

#[allow(dead_code)]
pub fn level_color(level: &log::LogLevel, msg: &str) -> String
{
    match level
    {
        &LogLevel::Error => msg.red(),
        &LogLevel::Warn  => msg.yellow(),
        &LogLevel::Info  => msg.green(),
        &LogLevel::Debug => msg.green(),
        &LogLevel::Trace => msg.magenta(),
    }.bold().to_string()
}
