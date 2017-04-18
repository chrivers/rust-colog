extern crate log;
extern crate colored;
extern crate env_logger;

use std::env;
#[allow(unused_imports)]
use log::LogLevelFilter;
use env_logger::LogBuilder;

mod format;
mod color;

#[allow(dead_code)]
pub fn builder() -> LogBuilder
{
    let mut builder = LogBuilder::new();
    builder.format(format::format);
    builder.filter(None, LogLevelFilter::Info);
    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }
    builder
}

pub fn init()
{
    builder().init().unwrap()
}
