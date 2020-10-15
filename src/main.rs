use clap::{App, Arg};

use std::process;

use tsur::Config;

fn main() {
    // let file = args.value_of("file");
    // let c = args.value_of("count chars");
    // println!("{:?}", file);
    // println!("{:?}", c);

    // let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Could not parse arguments: {}", err);
        process::exit(1);
    });

    match tsur::run(config) {
        Ok(i) => println!("{:#?}", i),
        _ => println!("U ducked up"),
    }
}
