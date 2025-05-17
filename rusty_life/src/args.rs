use clap::{Command, Arg, ArgMatches};

pub fn parse_args() -> ArgMatches {
    let matches = Command::new("RustyLifeCL")
        .author("Sebastian Pucher | 2025")
        .version("0.1.0")
        .about("A tiny, effiecnt, tui demo of conway's game of life written in rust")
        .args([
            Arg::new("border_type")
                .help("please specify the width of the grid. Must be a postive number")
                .short('b')
                .long("border")
                .value_parser(clap::value_parser!(usize)) 
                .default_value("100"),
            Arg::new("color")
                .help("please specify the length of the grid. Must be a postive number")
                .short('c')
                .long("color")
                .value_parser(clap::value_parser!(usize)) 
                .default_value("200"),
        ])
    .get_matches();
    return matches
}

