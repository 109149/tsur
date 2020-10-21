//! # Tsur Args.
//! For handling terminal arguments.
use clap::{App, Arg, SubCommand};

use crate::consts;

// TODO: read dir
pub fn args() -> clap::ArgMatches<'static> {
    let res = App::new("tsur")
        .version(consts::VERSION)
        .author(consts::AUTHOR)
        .about(consts::ABOUT)
        .subcommand(
            SubCommand::with_name("count")
                .arg(
                    Arg::with_name("count_chars")
                        .short("c")
                        .long("count-chars")
                        .takes_value(false)
                        .required(false)
                        .help("Count characters"),
                )
                .arg(
                    Arg::with_name("count_words")
                        .short("w")
                        .long("count-words")
                        .takes_value(false)
                        .required(false)
                        .help("Count words"),
                )
                .arg(
                    Arg::with_name("sort_by")
                        .short("s")
                        .long("sort-by")
                        .takes_value(true)
                        .required(false)
                        .possible_value("vasc")
                        .possible_value("vdesc")
                        .possible_value("kasc")
                        .possible_value("kdesc")
                        .default_value("vasc")
                        .help("Sort by key/value"),
                ),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .min_values(1)
                .value_name("FILE")
                .help("File to read"),
        )
        .get_matches();
    res
}
