//! # Tsur Core.
//!
//! Core caller.
// #![crate_name = "tsur"]
use std::process;

use tsur::{args, config::config, run}; // whatever is in lib.rs

fn main() {
    // Config
    let config = config::Config::new(args::args()).unwrap_or_else(|err| {
        eprintln!("Could not parse arguments: {}", err);
        process::exit(1);
    });

    // Is this really an efficient way doin this?
    let mut result: Vec<(char, usize)> = vec![];
    let mut res = match run::run(&config) {
        Ok(i) => {
            let _ = i
                .into_iter()
                .map(|k| result.push((k.0, k.1)))
                .collect::<()>();
            result
        }
        Err(e) => {
            println!("Something went wrong: {}", e);
            process::exit(1);
        }
    };

    match &config.get_sort_by()[..] {
        "vasc" => res.sort_by(|a, b| a.1.cmp(&b.1)),  // val: asc
        "vdesc" => res.sort_by(|a, b| b.1.cmp(&a.1)), // val: desc
        "kasc" => res.sort_by(|a, b| a.0.cmp(&b.0)),  // key: asc
        "kdesc" => res.sort_by(|a, b| b.0.cmp(&a.0)), // key: desc
        _ => (),
    };
    print(&res);
}

/// Print helper.
///
/// # Arguments
///
/// * `v` - A reference to a vector of (char, usize) tuple
fn print(v: &Vec<(char, usize)>) {
    println!("{{");
    let _ = v
        .iter()
        .map(|x| {
            println!("\t{}: {}", x.0, x.1);
        })
        .collect::<()>();
    println!("}}");
}
