//! # Tsur Config.
//!
//! Config structure.
// use std::ops::Deref;

#[derive(Debug)]
pub struct Config {
    filename: Vec<String>,
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
            sort_by = match sub_cmd.is_present("sort_by") {
                true => sub_cmd.value_of("sort_by").unwrap().to_string(),
                false => sort_by,
            };
        }

        let filename = match args.values_of("file") {
            Some(f) => f.map(|x| x.into()).collect::<Vec<String>>(),
            None => return Err("No filename(s)"),
        };

        Ok(Config {
            filename,
            count_ascii_characters,
            sort_by,
        })
    }

    pub fn get_filename(&self) -> &Vec<String> {
        &self.filename
    }

    pub fn count_ascii_characters(&self) -> bool {
        self.count_ascii_characters
    }

    pub fn get_sort_by(&self) -> &String {
        &self.sort_by
    }
}
