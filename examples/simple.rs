#[macro_use]
extern crate log;

fn main() {
    colog::init();
    error!("error message");
    error!("error with fmt: {}", 42);
    warn!("warn message");
    info!("info message");
    debug!("debug message");
    trace!("trace message");

    info!("multi line demonstration\nhere");
    info!("more\nmulti\nline\nhere\nhere");
}
