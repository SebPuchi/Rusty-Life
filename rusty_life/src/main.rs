mod grid;
mod args;
mod display;

use args::parse_args;
use grid::LifeGrid;
use display::Display;

fn main() {
    let user_matches = parse_args();
    // Safest - unwrap() is fine here since we have a default value
    let f_grid = LifeGrid::new(*user_matches.get_one::<usize>("grid_width").unwrap(), *user_matches.get_one::<usize>("grid_length").unwrap());
    let n_grid = LifeGrid::new(*user_matches.get_one::<usize>("grid_width").unwrap(), *user_matches.get_one::<usize>("grid_length").unwrap());
    println!("Grid width: {}, length: {}", n_grid.width, n_grid.length);
}

