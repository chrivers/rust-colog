use log::LogRecord;
use log::LogLevel;
use color;
use colored::*;

fn level_token(level: &LogLevel) -> &str
{
    match *level
    {
        LogLevel::Error => "E",
        LogLevel::Warn  => "W",
        LogLevel::Info  => "*",
        LogLevel::Debug => "D",
        LogLevel::Trace => "T",
    }
}

fn prefix_token(level: &LogLevel) -> String
{
    format!("{}{}{}", "[".blue().bold(), color::level_color(level, level_token(level)), "]".blue().bold())
}

pub fn format(record: &LogRecord) -> String
{
    let sep = format!("\n{} ", " | ".white().bold());
    format!(
        "{} {}",
        prefix_token(&record.level()),
        format!("{}", record.args()).replace("\n", &sep),
    )
}
