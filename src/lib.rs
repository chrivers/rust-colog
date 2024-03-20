use std::env;
use log::LevelFilter;
use env_logger::Builder;

pub mod format;

pub fn builder() -> Builder
{
    let mut builder = Builder::new();
    builder.format(format::format);
    builder.filter(None, LevelFilter::Info);
    if let Ok(rust_log) = env::var("RUST_LOG") {
        builder.parse_filters(&rust_log);
    }
    builder
}

pub fn init()
{
    builder().init()
}
