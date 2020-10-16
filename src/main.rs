//! # Tsur Core.
//!
//! Core caller.
use std::process;

use tsur::{args, config, run}; // whatever is in lib.rs

fn main() {
    let config = config::Config::new(args::args()).unwrap_or_else(|err| {
        eprintln!("Could not parse arguments: {}", err);
        process::exit(1);
    });

    match run::run(config) {
        Ok(i) => println!("{:#?}", i),
        _ => println!("U ducked up"),
    }
}
