use array2d::{Array2D, Error};

//Grid Logic
pub struct LifeGrid {
    pub height: u16,
    pub width: u16,
    cells: Array2D,
}

impl LifeGrid {
    pub fn new(height: u16, width: u16) -> Self {
        let array = Array2D::filled_with(false, height, width);
         Self { 
             height, 
             width,
             cells: array 
         }
     }
}

