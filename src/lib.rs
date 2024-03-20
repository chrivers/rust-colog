use std::env;
use log::LevelFilter;
use env_logger::Builder;

pub mod format;

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
