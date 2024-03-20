# Simple colored logger for rust #

The `colog` library is a simple formatter backend for the standard
rust logging system (in the `log` crate).

## Getting started ##

```rust
use log::{error, warn, info, debug, trace};

fn main() {
    // Quick start: use default initialization
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
```

This results in the following terminal output:

![demo screenshot from terminal](https://raw.githubusercontent.com/chrivers/rust-colog/master/screenshot.png)

## Custom styling ##

It's possible to override all colors and styles of `colog`.

See the following examples:

 - [examples/custom-level-colors.rs](examples/custom-level-colors.rs)
 - [examples/custom-level-colors.rs](examples/custom-level-colors.rs)

Also be sure to read the documentation (`cargo doc --open`) for a detailed description.

## Known issues and improvements ##

There are no known, serious, unsolved issues.

Patches welcome :)

## License ##

This project is licensed under the LGPLv3. See the file `LICENSE` for
details.
