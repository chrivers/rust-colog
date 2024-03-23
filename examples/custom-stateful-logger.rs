#[macro_use]
extern crate log;

use colored::Colorize;

use std::{io::Error, sync::Mutex};

use colog::format::CologStyle;
use env_logger::{fmt::Formatter, Builder};
use log::{Level, LevelFilter, Record};

#[derive(Default)]
pub struct CustomStatefulLogger {
    line: Mutex<usize>,
}

impl CologStyle for CustomStatefulLogger {
    fn level_token(&self, level: &Level) -> &str {
        match *level {
            Level::Error => "ERR",
            Level::Warn => "WRN",
            Level::Info => "INF",
            Level::Debug => "DBG",
            Level::Trace => "TRC",
        }
    }

    fn prefix_token(&self, level: &Level) -> String {
        let line = format!("[Log event {:4}]", self.line.lock().unwrap());
        format!(
            "{} {}",
            line.bright_white().on_blue(),
            colog::format::default_prefix_token(self, level)
        )
    }

    fn format(&self, buf: &mut Formatter, record: &Record<'_>) -> Result<(), Error> {
        *self.line.lock().unwrap() += 1;
        colog::format::default_format(self, buf, record)
    }
}

fn main() {
    let mut builder = Builder::new();
    builder.format(colog::formatter(CustomStatefulLogger::default()));
    builder.filter(None, LevelFilter::Trace);
    builder.init();
    error!("error message");
    error!("error with fmt: {}", 42);
    warn!("warn message");
    info!("info1 message");
    info!("info2 message");
    info!("info3 message");
    debug!("debug message");
    trace!("trace message");
}
