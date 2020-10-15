//! # Tsur Core.
//!
//! Core caller.
use std::process;

use tsur_args;
use tsur_config;
use tsur_run;

fn main() {
    let config = tsur_config::Config::new(tsur_args::args()).unwrap_or_else(|err| {
        eprintln!("Could not parse arguments: {}", err);
        process::exit(1);
    });

    match tsur_run::run(config) {
        Ok(i) => println!("{:#?}", i),
        _ => println!("U ducked up"),
    }
}
