use log::Level;
use colored::*;

#[allow(dead_code)]
pub fn level_color(level: &log::Level, msg: &str) -> String
{
    match level
    {
        Level::Error => msg.red(),
        Level::Warn  => msg.yellow(),
        Level::Info  => msg.green(),
        Level::Debug => msg.green(),
        Level::Trace => msg.magenta(),
    }.bold().to_string()
}
