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
        let array = Array2D::filled_with(false, (height * 2).into(), width.into());
        let adjusted_height = (height *2);
        let adjusted_width = (width);
         Self { 
            height: adjusted_height, 
            width: adjusted_width,
            cells: array, 
            generation: 0
         }
     }

    pub fn build_coords(&self) -> Vec<(f64, f64)> {
        let mut live_cells = Vec::new();

        for i in 0..self.generation {
            if i < self.width{ 
                live_cells.push((i as f64, 0.0));
            }
            if i < self.height{ 
                live_cells.push((0.0, i as f64));   
            }
        }


        live_cells
    }
}

