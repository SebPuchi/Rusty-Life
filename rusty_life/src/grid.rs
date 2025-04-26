use array2d::{Array2D, Error};

//Grid Logic
pub struct LifeGrid {
    pub x: u16,
    pub y: u16,
    pub height: u16,
    pub width: u16,
    pub generation: u16,
    cells: Array2D<bool>,
}

impl LifeGrid {
    pub fn new(x: u16, y: u16, height: u16, width: u16) -> Self {
        let array = Array2D::filled_with(false, (height * 2).into(), width.into());
         Self { 
            x,
            y,
            height, 
            width,
            cells: array, 
            generation: 0
         }
     }

    pub fn build_coords(&self) -> Vec<(f64, f64)> {
        let mut live_cells = Vec::new();

        for i in 0..self.generation {
                live_cells.push((i as f64, 0.0));          
                live_cells.push((0.0, i as f64));   
            }


        live_cells
    }
}

