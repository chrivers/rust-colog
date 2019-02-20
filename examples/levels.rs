#[macro_use]
extern crate log;
extern crate colog;

fn main()
{
    let mut clog = colog::builder();
    clog.filter(None, log::LevelFilter::Warn);
    clog.init();
    error!("error message");
    error!("error with fmt: {}", 42);
    warn!("warn message");

    /*
     * NOTE:
     *
     * The following log lines will not be printed, because the filter
     * is set to LevelFilter::Warn
     */
    info!("info message");
    debug!("debug message");
    trace!("trace message");
}
