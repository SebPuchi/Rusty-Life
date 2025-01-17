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
             //Add randomization fucntion
             cells: vec![false; width * length]
         }
     }

}

impl Default for LifeGrid {
    fn default() -> Self {
        let width = 100;
        let length = 50;
        Self {
            width,
            length,
            //Randomization function
            cells: vec![false; width * length]
        }
    }
}
