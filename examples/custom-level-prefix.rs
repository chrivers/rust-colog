#[macro_use]
extern crate log;

use colog::format::CologStyle;
use colored::Colorize;
use env_logger::Builder;
use log::{Level, LevelFilter};

pub struct CustomPrefixToken;

impl CologStyle for CustomPrefixToken {
    fn prefix_token(&self, level: &Level) -> String {
        format!(
            "{}{}{}",
            "| ".blue().bold(),
            self.level_color(level, self.level_token(level)),
            " -->".blue().bold()
        )
    }
}

fn main() {
    let mut builder = Builder::new();
    builder.format(colog::formatter(CustomPrefixToken));
    builder.filter(None, LevelFilter::Trace);
    builder.init();
    error!("error message");
    error!("error with fmt: {}", 42);
    warn!("warn message");
    info!("info message");
    debug!("debug message");
    trace!("trace message");
}
