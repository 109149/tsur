pub fn args() {
    let args = App::new("tRust")
        .version("0.1.0")
        .author("109149 <109149qwe@gmail.com>")
        .about("Just Rusting...You know...O_O")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .value_name("FILE")
                .help("File to read"),
        )
        .arg(
            Arg::with_name("mode")
                .short("m")
                .long("mode")
                .takes_value(true)
                .required(true)
                .possible_value("count chars")
                .possible_value("count words")
                .help("Select modes"),
        )
        .arg(
            Arg::with_name("bool_flag")
                .short("bf")
                .long("bool-flag")
                .takes_value(false)
                .help("QWE"),
        )
        .get_matches();
}
