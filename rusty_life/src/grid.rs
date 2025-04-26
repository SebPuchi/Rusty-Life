//Grid Logic
pub struct LifeGrid {
    pub width: usize,
    pub length: usize,
    cells: Vec<bool>,
}

impl LifeGrid {
    pub fn new(width: usize, length: usize) -> Self {
         Self { 
             width, 
             length,
             //Add randomization funcion
             cells: vec![false; width * length]
         }
     }

}

