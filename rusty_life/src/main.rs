mod grid;

use grid::LifeGrid;
use clap::Parser;


fn main() {
    let grid1 = LifeGrid::new(100, 100);
    let grid2 = LifeGrid::new(200, 200);
    let grid3 = LifeGrid::default();
    println!("custom_grid: width = {}, height = {}", grid1.width, grid1.height);
    println!("default_grid: width = {}, height = {}", grid2.width, grid2.height);
}
