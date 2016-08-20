#[macro_use]
extern crate log;
extern crate colored;
extern crate env_logger;

use std::env;
use log::LogLevelFilter;
use env_logger::LogBuilder;

mod format;
mod color;

#[allow(dead_code)]
pub fn init() {
    let mut builder = LogBuilder::new();
    builder.format(format::format).filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
       builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init().unwrap();
}
