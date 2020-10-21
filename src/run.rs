//! # Tsur Run.
//!
//! Runner.
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process;

use crate::ascii::ascii::ASCII;
use crate::config::config::Config;

pub fn run(config: &Config) -> Result<HashMap<char, usize>, Box<dyn Error>> {
    // let content = fs::read_to_string(&config.get_filename())?;
    let content = config
        .get_filename()
        .iter()
        .map(|x| match fs::read_to_string(x) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Can't read file(s): {}", e);
                process::exit(1);
            }
        })
        .collect::<String>();
    let ascii = ASCII::new();

    if config.count_ascii_characters() {
        Ok(ascii.count_ascii_characters(content.as_str()))
    } else {
        println!("Can't count characters");
        process::exit(1);
    }
}
