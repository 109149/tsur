//! # Tsur Config.
//!
//! Config structure.
#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub count_ascii_characters: bool,
}

impl Config {
    pub fn new(args: clap::ArgMatches) -> Result<Config, &'static str> {
        let count_ascii_characters = match args.value_of("mode") {
            Some(v) => {
                if v == "count chars" {
                    true
                } else {
                    false
                }
            }
            _ => false,
        };
        // let for_fun = args.is_present("bool_flag");

        let filename = match args.value_of("file") {
            Some(f) => f.to_string(),
            None => return Err("No filename"),
        };

        Ok(Config {
            filename,
            count_ascii_characters,
        })
    }
}
