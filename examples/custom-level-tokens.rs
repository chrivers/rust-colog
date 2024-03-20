#[macro_use]
extern crate log;

use colog::format::CologStyle;
use env_logger::Builder;
use log::{Level, LevelFilter};

pub struct CustomLevelToken;

impl CologStyle for CustomLevelToken {
    fn level_token(&self, level: &Level) -> &str {
        match *level {
            Level::Error => "ERR",
            Level::Warn => "WRN",
            Level::Info => "INF",
            Level::Debug => "DBG",
            Level::Trace => "TRC",
        }
    }
}

fn main() {
    let mut builder = Builder::new();
    builder.format(|buf, rec| CustomLevelToken.format(buf, rec));
    builder.filter(None, LevelFilter::Trace);
    builder.init();
    error!("error message");
    error!("error with fmt: {}", 42);
    warn!("warn message");
    info!("info message");
    debug!("debug message");
    trace!("trace message");
}
