//! # Tsur Config.
//!
//! Config structure.
#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub count_ascii_characters: bool,
    pub sort_by: String,
}

impl Config {
    pub fn new(args: clap::ArgMatches) -> Result<Config, &'static str> {
        let mut count_ascii_characters = true; // default
        let mut sort_by = String::from("vasc");
        if let Some(sub_cmd) = args.subcommand_matches("count") {
            // let sub_args: Vec<_> = sub_arg.args.iter().map(|x| x.0).collect();

            count_ascii_characters = sub_cmd.is_present("count_chars");
            sort_by = sub_cmd.value_of("sort_by").unwrap().to_string();
        }

        let filename = match args.value_of("file") {
            Some(f) => f.to_string(),
            None => return Err("No filename"),
        };

        Ok(Config {
            filename,
            count_ascii_characters,
            sort_by,
        })
    }
}
