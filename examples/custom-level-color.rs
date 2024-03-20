#[macro_use]
extern crate log;

use colog::format::CologStyle;
use colored::Colorize;
use env_logger::Builder;
use log::{Level, LevelFilter};

pub struct CustomLevelColors {}

impl CologStyle for CustomLevelColors {
    fn level_color(&self, level: &log::Level, msg: &str) -> String {
        match level {
            Level::Error => msg.on_red(),
            Level::Warn => msg.on_yellow(),
            Level::Info => msg.on_green(),
            Level::Debug => msg.on_blue(),
            Level::Trace => msg.on_purple(),
        }
        .bright_white()
        .to_string()
    }
}

fn main() {
    let style = CustomLevelColors {};
    let mut builder = Builder::new();
    builder.format(move |buf, rec| style.format(buf, rec));
    builder.filter(None, LevelFilter::Trace);
    builder.init();
    error!("error message");
    error!("error with fmt: {}", 42);
    warn!("warn message");
    info!("info message");
    debug!("debug message");
    trace!("trace message");
}
