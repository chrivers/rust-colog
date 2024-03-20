//! Functions and types for formatting log messages
//!
use std::io::{Error, Write};

use colored::*;
use env_logger::fmt::Formatter;
use log::{Level, Record};

/// A style for [`colog`](crate)
///
/// All functions on this trait come with a provided default implementation.
/// This is how to the default style is provided.
///
/// To create a custom style, make a new type, and implement [`CologStyle`] on
/// it, overriding one or more of the default implementations on the trait.
///

/// ```rust
/// #[macro_use]
/// extern crate log;
/// use colog::format::CologStyle;
/// use env_logger::Builder;
/// use log::{Level, LevelFilter};
///
/// // unless we want state, we don't need any fields on our style type
/// pub struct CustomLevelToken;
///
/// // implement CologStyle on our type, and override `level_token`
/// impl CologStyle for CustomLevelToken {
///     fn level_token(&self, level: &Level) -> &str {
///         match *level {
///             Level::Error => "ERR",
///             Level::Warn => "WRN",
///             Level::Info => "INF",
///             Level::Debug => "DBG",
///             Level::Trace => "TRC",
///         }
///     }
/// }
///
/// fn main() {
///     let mut builder = Builder::new();
///
///     // this is where we enable our custom styling
///     builder.format(move |buf, rec| CustomLevelToken.format(buf, rec));
///
///     // set a custom filter level
///     builder.filter(None, LevelFilter::Trace);
///
///     // initialize the logger
///     builder.init();
///
///     error!("error message");
///     error!("error with fmt: {}", 42);
///     warn!("warn message");
///     info!("info message");
///     debug!("debug message");
///     trace!("trace message");
/// }
/// ```


pub trait CologStyle {
    fn level_color(&self, level: &log::Level, msg: &str) -> String {
        match level {
            Level::Error => msg.red(),
            Level::Warn => msg.yellow(),
            Level::Info => msg.green(),
            Level::Debug => msg.green(),
            Level::Trace => msg.magenta(),
        }
        .bold()
        .to_string()
    }

    fn level_token(&self, level: &Level) -> &str {
        match *level {
            Level::Error => "E",
            Level::Warn => "W",
            Level::Info => "*",
            Level::Debug => "D",
            Level::Trace => "T",
        }
    }

    fn prefix_token(&self, level: &Level) -> String {
        format!(
            "{}{}{}",
            "[".blue().bold(),
            self.level_color(level, self.level_token(level)),
            "]".blue().bold()
        )
    }

    fn format(&self, buf: &mut Formatter, record: &Record<'_>) -> Result<(), Error> {
        let sep = format!("\n{} ", " | ".white().bold());
        writeln!(
            buf,
            "{} {}",
            self.prefix_token(&record.level()),
            record.args().to_string().replace('\n', &sep),
        )
    }
}

pub struct DefaultCologStyle;

impl CologStyle for DefaultCologStyle {}
