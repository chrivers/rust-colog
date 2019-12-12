use env_logger::fmt::Formatter;
use log::Record;
use log::Level;
use crate::color;
use colored::*;
use std::io::{Error, Write};

fn level_token(level: &Level) -> &str
{
    match *level
    {
        Level::Error => "E",
        Level::Warn  => "W",
        Level::Info  => "*",
        Level::Debug => "D",
        Level::Trace => "T",
    }
}

fn prefix_token(level: &Level) -> String
{
    format!("{}{}{}", "[".blue().bold(), color::level_color(level, level_token(level)), "]".blue().bold())
}

pub fn format(buf: &mut Formatter, record: &Record<'_>) -> Result<(), Error>
{
    let sep = format!("\n{} ", " | ".white().bold());
    writeln!(
        buf,
        "{} {}",
        prefix_token(&record.level()),
        format!("{}", record.args()).replace("\n", &sep),
    )
}
