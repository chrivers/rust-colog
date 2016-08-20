#[macro_use]
extern crate log;
extern crate colog;

fn main()
{
    drop(colog::init());
    error!("error message");
    error!("error with fmt: {}", 42);
    warn!("warn message");
    info!("info message");
    debug!("debug message");
    trace!("trace message");
}
