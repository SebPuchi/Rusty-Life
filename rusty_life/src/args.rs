use clap::{Command, Arg, ArgMatches};


pub fn parse_args() -> ArgMatches {
    let matches = Command::new("My Program")
        .author("Sebastian Pucher | 2025")
        .version("0.1.0")
        .about("A tiny, terminal-based demonstration of Conway's Game of Life in Rust.")
        .args([
            Arg::new("grid_width")
                .help("the input file to use")
                .short('w')
                .long("gridwidth"),
            Arg::new("grid_length")
                .help("the input file to use")
                .short('l')
                .long("gridlength"),
        ])
    .get_matches();
    matches
}

