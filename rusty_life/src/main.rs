mod grid;
mod args;

use args::parse_args;
use grid::LifeGrid;

fn main() {
    let grid1 = LifeGrid::new(100, 100);
    let grid2 = LifeGrid::new(200, 200);
    parse_args();
    println!("grid1: width = {}, height = {}", grid1.width, grid1.height);
    println!("grid2: width = {}, height = {}", grid2.width, grid2.height);
    //println!("width arg = {}", args.grid_width);
//    println!("height arg = {}", args.grid_height);
}
