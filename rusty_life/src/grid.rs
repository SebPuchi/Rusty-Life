//Grid Logic

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    
    pub fn new(width: usize, height: usize, cells: Vec<bool>) -> Self {
        Self { 
            width, 
            height,
            cells 
        }
    }

impl Default for Grid {
    fn default() -> Self {
        Self {
            width: 100,
            height: 50,
            cells
        }
    }
}
