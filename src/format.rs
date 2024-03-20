use colored::*;
use env_logger::fmt::Formatter;
use log::Level;
use log::Record;
use std::io::{Error, Write};

pub fn level_color(level: &log::Level, msg: &str) -> String {
    match level {
        Level::Error => msg.red(),
        Level::Warn => msg.yellow(),
        Level::Info => msg.green(),
        Level::Debug => msg.green(),
        Level::Trace => msg.magenta(),
    }
    .bold()
    .to_string()
}

fn level_token(level: &Level) -> &str {
    match *level {
        Level::Error => "E",
        Level::Warn => "W",
        Level::Info => "*",
        Level::Debug => "D",
        Level::Trace => "T",
    }
}

fn prefix_token(level: &Level) -> String {
    format!(
        "{}{}{}",
        "[".blue().bold(),
        level_color(level, level_token(level)),
        "]".blue().bold()
    )
}

pub fn format(buf: &mut Formatter, record: &Record<'_>) -> Result<(), Error> {
    let sep = format!("\n{} ", " | ".white().bold());
    writeln!(
        buf,
        "{} {}",
        prefix_token(&record.level()),
        format!("{}", record.args()).replace('\n', &sep),
    )
}
