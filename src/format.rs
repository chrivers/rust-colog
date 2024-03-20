use std::io::{Error, Write};

use colored::*;
use env_logger::fmt::Formatter;
use log::{Level, Record};

pub trait CologStyle {
    fn level_color(&self, level: &log::Level, msg: &str) -> String {
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

    fn level_token(&self, level: &Level) -> &str {
        match *level {
            Level::Error => "E",
            Level::Warn => "W",
            Level::Info => "*",
            Level::Debug => "D",
            Level::Trace => "T",
        }
    }

    fn prefix_token(&self, level: &Level) -> String {
        format!(
            "{}{}{}",
            "[".blue().bold(),
            self.level_color(level, self.level_token(level)),
            "]".blue().bold()
        )
    }

    fn format(&self, buf: &mut Formatter, record: &Record<'_>) -> Result<(), Error> {
        let sep = format!("\n{} ", " | ".white().bold());
        writeln!(
            buf,
            "{} {}",
            self.prefix_token(&record.level()),
            record.args().to_string().replace('\n', &sep),
        )
    }
}

pub struct DefaultCologStyle {}

impl CologStyle for DefaultCologStyle {}

/// Default colog formatting style
pub static DEFAULT: DefaultCologStyle = DefaultCologStyle {};
