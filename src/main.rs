use std::env;

use log::LevelFilter;

fn setup_logger() {
    let mut builder = &mut env_logger::builder();

    // check if user has set -v
    let mut pargs = pico_args::Arguments::from_env();
    let verbose = pargs.contains(["-v", "--verbose"]);

    // do something only if env var has not been set explicitely by user
    if env::var("RUST_LOG").is_err() {
        let wanted_level = if verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        };

        // set level **only for our own package**
        let package_mod = module_path!().split("::").next().unwrap_or_default();
        builder = builder.filter_module(package_mod, wanted_level);
    } else {
        if verbose {
            eprintln!("RUST_LOG env var has been set by user, ignoring -v");
        }
    }

    builder.init();
}

fn main() {
    setup_logger();

    log::error!("This is an error message");
    log::warn!("This is a warning message");
    log::info!("This is an info message");
    log::debug!("This is a debug message (not shown without -v)");
    log::trace!("This is a trace message (not shown without RUST_LOG=trace)");

    // reqwest debug message should not be printed when -v is set (but will be shown with RUST_LOG=debug !)
    reqwest::blocking::get("https://www.rust-lang.org")
        .expect("Failed to send request")
        .text()
        .expect("Failed to get response body");
}
