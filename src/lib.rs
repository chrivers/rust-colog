#![doc = include_str!("../README.md")]

use env_logger::Builder;
use log::LevelFilter;
use std::env;

pub mod format;

use format::CologStyle;

/// Returns a [`env_logger::Builder`] that is configured to use [`crate`]
/// formatting for its output.
///
/// This can be used as a building block to integrate into existing
/// [`env_logger`] applications.
///
/// If desired, these steps can be performed manually, like so:
///
/// ```rust
/// use colog::format::CologStyle;
/// let mut builder = env_logger::Builder::new();
/// builder.format(|buf, rec| colog::format::DEFAULT.format(buf, rec));
/// /* further builder setup here.. */
/// builder.init();
/// log::info!("logging is ready");
/// ```
pub fn basic_builder() -> Builder {
    let mut builder = Builder::new();
    builder.format(|buf, rec| format::DEFAULT.format(buf, rec));
    builder
}

/// Opinionated builder, with [`colog`](crate) defaults.
///
/// This function returns a builder that:
///  - Uses [`colog`](crate) formatting
///  - Presents messages at severity [`LevelFilter::Info`] and up
///  - Optionally uses `RUST_LOG` environment settings
///
/// ```rust
/// let mut builder = colog::default_builder();
/// /* further builder setup here.. */
/// builder.init();
/// log::info!("logging is ready");
/// ```
pub fn default_builder() -> Builder {
    let mut builder = basic_builder();
    builder.filter(None, LevelFilter::Info);
    if let Ok(rust_log) = env::var("RUST_LOG") {
        builder.parse_filters(&rust_log);
    }
    builder
}

/// Deprecated. Use [`default_builder`] instead (see also [`basic_builder`])
#[deprecated(note = "Use `default_builder` instead")]
pub fn builder() -> Builder {
    default_builder()
}

/// Convenience function to initialize logging.
///
/// This function constructs the default builder [`default_builder`] and
/// initializes it, without any custom options.
///
/// If more flexibility is needed, see [`default_builder`] or [`basic_builder`]
pub fn init() {
    default_builder().init()
}
