mod grid;
mod args;
mod app;

use args::parse_args;
use grid::LifeGrid;
use app::App;

fn main() {
    let user_matches = parse_args();
    // Safest - unwrap() is fine here since we have a default value
    let f_grid = LifeGrid::new(*user_matches.get_one::<usize>("grid_width").unwrap(), *user_matches.get_one::<usize>("grid_length").unwrap());
    let n_grid = LifeGrid::new(*user_matches.get_one::<usize>("grid_width").unwrap(), *user_matches.get_one::<usize>("grid_length").unwrap());
    println!("Grid width: {}, length: {}", n_grid.width, n_grid.length);
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

