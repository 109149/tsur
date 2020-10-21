//! # Tsur Config.
//!
//! Config structure.
// use std::ops::Deref;

#[derive(Debug)]
pub struct Config {
    filename: String,
    count_ascii_characters: bool,
    sort_by: String,
}

// impl Deref for Config {
//     type Target = String;

//     fn deref(&self) -> &Self::Target {
//         &self.filename
//     }
// }

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

    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn count_ascii_characters(&self) -> bool {
        self.count_ascii_characters
    }

    pub fn get_sort_by(&self) -> &String {
        &self.sort_by
    }
}
