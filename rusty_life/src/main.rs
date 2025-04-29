mod grid;
mod args;
mod app;

use std::io;
use args::parse_args;
use grid::LifeGrid;
use app::App;

fn main() -> io::Result<()> {
    //let user_matches = parse_args();
    //let f_grid = LifeGrid::new(*user_matches.get_one::<usize>("grid_width").unwrap(), *user_matches.get_one::<usize>("grid_length").unwrap());
    let mut terminal = ratatui::init();
    let mut app_result = App::new(&mut terminal);
    app_result.run(&mut terminal);
    ratatui::restore();

    app_result.status
}

