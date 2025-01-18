use crate::grid::LifeGrid;

// Display Logic
pub struct Display {
    pub grid: LifeGrid,
}

impl Display {
    pub fn new(grid: LifeGrid) -> Self {
         Self { 
             grid, 
         }
     }

}

