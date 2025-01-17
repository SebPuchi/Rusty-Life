mod grid;
mod args;

use args::parse_args;
use grid::LifeGrid;

fn main() {
    let user_matches = parse_args();
    println!("width: {:?}", user_matches.get_one::<usize>("grid_width"));
    println!("length: {:?}", user_matches.get_one::<usize>("grid_length"));
}
