extern crate log;
extern crate colored;
extern crate env_logger;

use std::env;
use log::LevelFilter;
use env_logger::Builder;

mod format;
mod color;

pub fn builder() -> Builder
{
    let mut builder = Builder::new();
    builder.format(format::format);
    builder.filter(None, LevelFilter::Info);
    if env::var("RUST_LOG").is_ok() {
        builder.parse_filters(&env::var("RUST_LOG").unwrap());
    }
    builder
}

pub fn init()
{
    builder().init()
}
