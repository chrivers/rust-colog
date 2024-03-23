//! Functions and types for formatting log messages
//!
use std::io::{Error, Write};

use colored::*;
use env_logger::fmt::Formatter;
use log::{Level, Record};

/// Customizable styles for [`colog`](crate)
///
/// All functions on this trait come with a provided default implementation.
/// This is how to the default style is provided.
///
/// To create a custom style, make a new type which implements [`CologStyle`],
/// overriding one or more of the default implementations on the trait.
///
/// # Examples

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
///     builder.format(colog::formatter(CustomLevelToken));
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
    /// Format a message for a particular log level
    ///
    /// # Parameters
    ///
    /// - `level`: The log level of the message
    /// - `msg`: The message text
    ///
    /// # Returns
    ///
    /// A [`String`] which is formatted according to the level. Typically, this
    /// is done by wrapping the string in terminal color codes, appropriate for
    /// the log level.
    ///
    /// # Defaults
    ///
    /// See [`default_level_color`]

    fn level_color(&self, level: &log::Level, msg: &str) -> String {
        default_level_color(level, msg)
    }

    /// Provide a "token" for a particular log level
    ///
    /// # Parameters
    ///
    /// - `level`: The log level of the message
    ///
    /// # Returns
    ///
    /// A [`&str`] which represents the token name for the given log level.
    ///
    /// This is typically a single letter (`"W"` for warn, `"D"` for debug, etc), or a short
    /// string (`"WARN"`, `"DEBUG"`, etc)
    ///
    /// # Defaults
    ///
    /// See [`default_level_token`]

    fn level_token(&self, level: &Level) -> &str {
        default_level_token(level)
    }

    /// Construct the line prefix for a message of the given log level.
    ///
    /// This method is not typically overriden (rather [`Self::level_color`] or
    /// [`Self::level_token`] is specialized), but is available for
    /// customization
    ///
    /// # Defaults
    ///
    /// See [`default_prefix_token`]

    fn prefix_token(&self, level: &Level) -> String {
        default_prefix_token(self, level)
    }

    /// Returns the default line separator string, used when formatting
    /// multi-line log messages.
    ///
    /// When implementing a style where multi-line log messages should appear
    /// unchanged, override this method to return `"\n"`.
    ///
    /// # Returns
    ///
    /// A string which will replace `"\n"` in the log message.
    ///
    /// # Defaults
    ///
    /// `"\n"` + (`" | "` in bold white)

    fn line_separator(&self) -> String {
        format!("\n{} ", " | ".white().bold())
    }

    /// Top-level formatting function for log messages.
    ///
    /// This method is not typically overriden (rather [`Self::level_color`] or
    /// [`Self::level_token`] is specialized), but is available for
    /// customization.
    ///
    /// Overriding this method entirely changes the behavior of [`colog`](crate).
    ///
    /// # Defaults
    ///
    /// See [`default_format`]

    fn format(&self, buf: &mut Formatter, record: &Record<'_>) -> Result<(), Error> {
        default_format(self, buf, record)
    }
}

/// Default colog style. Can manually be enabled like so:
/// ```rust
/// use log::info;
/// use colog::format::{CologStyle, DefaultCologStyle};
///
/// let mut builder = env_logger::Builder::new();
///
/// builder.format(colog::formatter(DefaultCologStyle));
/// /* ... */
/// builder.init();
///
/// info!("logging ready");

pub struct DefaultCologStyle;

impl CologStyle for DefaultCologStyle {}

/// Format a message for a particular log level
///
/// # Parameters
///
/// - `level`: The log level of the message
/// - `msg`: The message text
///
/// # Returns
///
/// A [`String`] which is formatted according to the level. Typically, this
/// is done by wrapping the string in terminal color codes, appropriate for
/// the log level.
///
/// # Defaults
///
/// | Level            | Color   |
/// |------------------|---------|
/// | [`Level::Error`] | red     |
/// | [`Level::Warn`]  | yellow  |
/// | [`Level::Info`]  | green   |
/// | [`Level::Debug`] | green   |
/// | [`Level::Trace`] | magenta |

pub fn default_level_color(level: &log::Level, msg: &str) -> String {
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

/// Default implementation for [`CologStyle::level_token`]
///
/// # Parameters
///
/// - `level`: The log level of the message
///
/// # Returns
///
/// A [`&str`] which represents the token name for the given log level.
///
/// | Level            | Token |
/// |------------------|-------|
/// | [`Level::Error`] | `"E"` |
/// | [`Level::Warn`]  | `"W"` |
/// | [`Level::Info`]  | `"*"` |
/// | [`Level::Debug`] | `"D"` |
/// | [`Level::Trace`] | `"T"` |

pub fn default_level_token(level: &Level) -> &'static str {
    match level {
        Level::Error => "E",
        Level::Warn => "W",
        Level::Info => "*",
        Level::Debug => "D",
        Level::Trace => "T",
    }
}

/// Default implementation for [`CologStyle::prefix_token`]
///
/// # Returns
///
/// Formats the level token ([`style.level_token`]) using
/// [`style.level_color`], wrapped in `[` and `]` formatted in bold blue.

pub fn default_prefix_token(style: &(impl CologStyle + ?Sized), level: &Level) -> String {
    format!(
        "{}{}{}",
        "[".blue().bold(),
        style.level_color(level, style.level_token(level)),
        "]".blue().bold()
    )
}

/// Default implementation for [`CologStyle::format`]
///
/// # Returns
///
/// Inject [`style.line_separator`] prefix at newlines, and format result according to
/// [`style.prefix_token`].
///
/// (this is the default [`colog`](crate) style)

pub fn default_format(
    style: &(impl CologStyle + ?Sized),
    buf: &mut Formatter,
    record: &Record<'_>,
) -> Result<(), Error> {
    let sep = style.line_separator();
    let prefix = style.prefix_token(&record.level());
    writeln!(
        buf,
        "{} {}",
        prefix,
        record.args().to_string().replace('\n', &sep),
    )
}
