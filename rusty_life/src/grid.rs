//Grid Logic
pub struct LifeGrid {
    pub width: usize,
    pub height: usize,
    cells: Vec<bool>,
}

impl LifeGrid {
    pub fn new(width: usize, height: usize) -> Self {
         Self { 
             width, 
             height,
             //Add randomization fucntion
             cells: vec![false; width * height]
         }
     }

}

impl Default for LifeGrid {
    fn default() -> Self {
        let width = 100;
        let height = 50;
        Self {
            width,
            height,
            //Randomization function
            cells: vec![false; width * height]
        }
    }
}
