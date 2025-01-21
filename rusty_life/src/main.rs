mod grid;
mod args;
mod app;

use std::io;
use args::parse_args;
use grid::LifeGrid;
use app::App;

fn main() -> io::Result<()> {
    let user_matches = parse_args();
    // Safest - unwrap() is fine here since we have a default value
    let f_grid = LifeGrid::new(*user_matches.get_one::<usize>("grid_width").unwrap(), *user_matches.get_one::<usize>("grid_length").unwrap());
    let mut terminal = ratatui::init();
    let app_result = App::new(f_grid).run(&mut terminal);
    ratatui::restore();
    app_result
}

