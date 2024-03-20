#![doc = include_str!("../README.md")]

use env_logger::Builder;
use log::LevelFilter;
use std::env;

pub mod format;

pub fn basic_builder() -> Builder {
    let mut builder = Builder::new();
    builder.format(format::format);
    builder
}

pub fn default_builder() -> Builder {
    let mut builder = basic_builder();
    builder.filter(None, LevelFilter::Info);
    if let Ok(rust_log) = env::var("RUST_LOG") {
        builder.parse_filters(&rust_log);
    }
    builder
}

#[deprecated(note = "Use `default_builder` instead")]
pub fn builder() -> Builder {
    default_builder()
}

pub fn init() {
    default_builder().init()
}
