use array2d::{Array2D, Error};

//Grid Logic
pub struct LifeGrid {
    pub height: u16,
    pub width: u16,
    pub generation: u16,
    cells: Array2D<bool>,
}

impl LifeGrid {
    pub fn new(height: u16, width: u16) -> Self {
        let array = Array2D::filled_with(false, height.into(), width.into());
         Self { 
            height, 
            width,
            cells: array, 
            generation: 0
         }
     }

    pub fn build_coords(&self) -> Vec<(f64, f64)> {
            

    
    }
}

