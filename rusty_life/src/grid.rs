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
            //Add randomization fucntion
            cells: vec![false; width * height]
        }
    }

impl Default for Grid {
    fn default() -> Self {
        Self {
            width: 100,
            height: 50,
            //Randomization function
            cells: vec![false; width * height]
        }
    }
}
