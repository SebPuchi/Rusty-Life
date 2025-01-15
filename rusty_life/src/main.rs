use grid::Grid;

mod grid;
mod display;

fn main() {
    let custom_grid = Grid::new(200, 100);
    let default_grid: Grid = Grid::default();
    println!("custom_grid: width = {}, height = {}", custom_grid.width, custom_grid.height);
    println!("default_grid: width = {}, height = {}", default_grid.width, default_grid.height);
}
