//! # Colog: Simple colored logger for rust
//!
//! The `colog` library is a simple formatter backend for the standard
//! rust logging system (in the `log` crate).
//!
//! For convenience, [`colog`](crate) provides utility functions to help
//! initialize logging, while using some reasonable defaults.
//!
//! All these defaults can be controlled, using a few more lines of code.
//!
//! Example code:
//!  - `examples/simple.rs` (minimal example)
//!  - `examples/levels.rs` (custom log levels)
//!
//! ## Minimal example
//!
//! ```rust
//! use log::{error, warn, info, debug, trace};
//!
//! // Quick start: use default initialization
//! colog::init();
//!
//! error!("error message");
//! error!("error with fmt: {}", 42);
//! warn!("warn message");
//! info!("info message");
//! debug!("debug message"); // not printed (LogLevel::Info is the default level)
//! trace!("trace message"); // not printed (LogLevel::Info is the default level)
//!
//! // notice how multi-line comments are handled gracefully
//! info!("multi line demonstration\nhere");
//! info!("more\nmulti\nline\nhere\nhere");
//! ```
//!
//! This results in the following terminal output:
//!
//! ![demo screenshot from terminal](https://raw.githubusercontent.com/chrivers/rust-colog/master/screenshot.png)
//!
//! ## Custom styling ##
//!
//! All the styling of [`colog`](crate) can be overriden.
//!
//! The styling is provided by the trait [`CologStyle`], which provides default
//!  implementations for all methods, resulting in the default colog style.
//!
//! Example code:
//!   - `examples/custom-level-colors.rs`
//!   - `examples/custom-level-tokens.rs`

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
/// builder.format(|buf, rec| colog::format::DefaultCologStyle.format(buf, rec));
/// /* further builder setup here.. */
/// builder.init();
/// log::info!("logging is ready");
/// ```
pub fn basic_builder() -> Builder {
    let mut builder = Builder::new();
    builder.format(|buf, rec| format::DefaultCologStyle.format(buf, rec));
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
