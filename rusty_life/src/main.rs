mod grid;
mod args;

use args::parse_args;
use grid::LifeGrid;

fn main() {
    let user_matches = parse_args();
    // Safest - unwrap() is fine here since we have a default value
    let n_grid = LifeGrid::new(*user_matches.get_one::<usize>("grid_width").unwrap(), *user_matches.get_one::<usize>("grid_width").unwrap());
}

