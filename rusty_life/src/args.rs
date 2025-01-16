use clap::{command, Arg};


pub fn parse_args() {
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("grid_width")
            .short('w')
            .long("gridwidth"))
        .arg(Arg::new("grid_height")
            .short('l')
            .long("gridlength"))
        .get_matches();

}

