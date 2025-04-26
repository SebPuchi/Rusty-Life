use array2d::{Array2D, Error};

//Grid Logic
pub struct LifeGrid {
    pub width: usize,
    pub height: usize,
    cells: Vec<bool>,
}

impl LifeGrid {
    pub fn new(width: usize, length: usize) -> Self {
         Self { 
             width, 
             height,
             cells: vec![false; width * length]
         }
     }

}

