use array2d::{Array2D, Error};

//Grid Logic
pub struct LifeGrid {
    pub height: u16,
    pub width: u16,
    pub generation: u16,
    current_cells: Array2D<bool>,
    next_cells: Array2D<bool>,
}

impl LifeGrid {
    pub fn new(height: u16, width: u16) -> Self {
        let current = Array2D::filled_with(false, (height * 2).into(), width.into());
        let next = Array2D::filled_with(false, (height * 2).into(), width.into());
        let adjusted_height = (height *2);
        let adjusted_width = (width);

         Self { 
            height: adjusted_height, 
            width: adjusted_width,
            current_cells: current, 
            next_cells: next, 
            generation: 0
         }

     }
    
    pub fn evolve_next(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let live_neighbors = self.count_live_neighbors(row as usize, col as usize);

                let is_alive = *self.current_cells.get(row as usize, col as usize).unwrap_or(&false);
                let next_state = match (is_alive, live_neighbors) {
                    (true, 2) | (true, 3) => true, 
                    (false, 3) => true,            
                    _ => false,                    
                };
                self.next_cells.set(row as usize, col as usize, next_state).unwrap();
            }
        }

        std::mem::swap(&mut self.current_cells, &mut self.next_cells);

        self.generation += 1;
    }

    fn count_live_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for dr in [-1i32, 0, 1].iter() {
            for dc in [-1i32, 0, 1].iter() {
                if *dr == 0 && *dc == 0 {
                    continue; // Skip the cell itself
                }
                let nr = row as i32 + dr;
                let nc = col as i32 + dc;
                if nr >= 0 && nc >= 0 && (nr as usize) < self.height as usize && (nc as usize) < self.width as usize {
                    if *self.current_cells.get(nr as usize, nc as usize).unwrap_or(&false) {
                        count += 1;
                    }
                }
            }
        }
        count
    }   

    pub fn build_coords(&self) -> Vec<(f64, f64)> {
        let mut live_cells = Vec::new();

        for row in 0..self.height {
            for col in 0..self.width {
                if *self.current_cells.get(row as usize, col as usize).unwrap_or(&false) {
                    let x = col as f64;
                    let y = (self.height - 1 - row) as f64; // flip y-axis
                    live_cells.push((x, y));
                }
            }
        }

        live_cells
    }

    pub fn spawn_glider_center(&mut self) {
        let center_row = self.height as usize / 2;
        let center_col = self.width as usize / 2;

        let glider_coords = [
            (center_row, center_col + 1),
            (center_row + 1, center_col + 2),
            (center_row + 2, center_col),
            (center_row + 2, center_col + 1),
            (center_row + 2, center_col + 2),
        ];

        for (row, col) in glider_coords.iter() {
            if let Some(cell) = self.current_cells.get_mut(*row, *col) {
                *cell = true;
            }
        }
    }

    pub fn spawn_r_center(&mut self) {
        let center_row = self.height as usize / 2;
        let center_col = self.width as usize / 2;

        let r_coords = [
            (center_row, center_col),
            (center_row, center_col + 1),
            (center_row, center_col - 1),
            (center_row -1, center_col),
            (center_row +1, center_col + 1),
        ];

        for (row, col) in r_coords.iter() {
            if let Some(cell) = self.current_cells.get_mut(*row, *col) {
                *cell = true;
            }
        }
    }

}

