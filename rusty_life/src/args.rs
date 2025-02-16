use clap::{Command, Arg, ArgMatches};

pub fn parse_args() -> ArgMatches {
    let matches = Command::new("RustyLifeCL")
        .author("Sebastian Pucher | 2025")
        .version("0.1.0")
        .about("A tiny, terminal-based demonstration of Conway's Game of Life written in Rust")
        .args([
            Arg::new("grid_width")
                .help("please specify the width of the grid. Must be a postive number")
                .short('w')
                .long("gridwidth")
                .value_parser(clap::value_parser!(usize)) 
                .default_value("100"),
            Arg::new("grid_length")
                .help("please specify the length of the grid. Must be a postive number")
                .short('l')
                .long("gridlength")
                .value_parser(clap::value_parser!(usize)) 
                .default_value("200"),
        ])
    .get_matches();
    return matches
}

