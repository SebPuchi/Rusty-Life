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
                    let y =  row as f64; 
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
            (center_row +1, center_col +1),
            (center_row, center_col-1),
            (center_row+ 1, center_col),
            (center_row - 1, center_col),
        ];

        for (row, col) in r_coords.iter() {
            if let Some(cell) = self.current_cells.get_mut(*row, *col) {
                *cell = true;
            }
        }
    }

    pub fn spawn_bc_center(&mut self) {
        let center_row = self.height as usize / 2;
        let center_col = self.width as usize / 2;

        let r_coords = [
            ((center_row+10), (center_col-50)),
            ((center_row+11), (center_col-50)),
            ((center_row+11), (center_col-49)),
            ((center_row+11), (center_col-47)),
            ((center_row+11), (center_col-46)),
            ((center_row+10), (center_col-45)),
            ((center_row+11), (center_col-44)),
            ((center_row+11), (center_col-43)),
            ((center_row+11), (center_col-42)),


            ((center_row+12), (center_col-42)),
            ((center_row+13), (center_col-42)),
            ((center_row+14), (center_col-42)),
            ((center_row+15), (center_col-42)),
            ((center_row+12), (center_col-43)),
            ((center_row+13), (center_col-43)),
            ((center_row+14), (center_col-43)),
            ((center_row+15), (center_col-43)),

            ((center_row+15), (center_col-49)),
            ((center_row+16), (center_col-49)),
            ((center_row+16), (center_col-51)),


            ((center_row+15), (center_col-48)),
            ((center_row+16), (center_col-48)),
            ((center_row+17), (center_col-48)),
            ((center_row+18), (center_col-48)),
            ((center_row+19), (center_col-48)),
            ((center_row+20), (center_col-48)),
            ((center_row+20), (center_col-49)),
            ((center_row+20), (center_col-51)),
            ((center_row+20), (center_col-52)),

            ((center_row+20), (center_col-47)),
            ((center_row+19), (center_col-47)),
            ((center_row+18), (center_col-47)),
            ((center_row+17), (center_col-47)),
            ((center_row+16), (center_col-47)),
            ((center_row+15), (center_col-47)),
            ((center_row+14), (center_col-47)),
            ((center_row+13), (center_col-47)),
            ((center_row+12), (center_col-47)),

            ((center_row+14), (center_col-48)),
            ((center_row+13), (center_col-48)),
            ((center_row+12), (center_col-48)),


            ((center_row+21), (center_col-47)),
            ((center_row+21), (center_col-46)),
            ((center_row+21), (center_col-45)),
            ((center_row+21), (center_col-44)),

            ((center_row+20), (center_col-44)),
            ((center_row+19), (center_col-44)),
            ((center_row+18), (center_col-44)),
            ((center_row+20), (center_col-43)),
            ((center_row+19), (center_col-43)),
            ((center_row+18), (center_col-43)),
            ((center_row+17), (center_col-45)),
            ((center_row+16), (center_col-43)),
            ((center_row+16), (center_col-44)),

            ((center_row+12), (center_col-50)),
            ((center_row+13), (center_col-50)),
            ((center_row+14), (center_col-50)),
            ((center_row+15), (center_col-50)),
            ((center_row+16), (center_col-50)),
            ((center_row+17), (center_col-50)),
            ((center_row+18), (center_col-50)),
            ((center_row+19), (center_col-50)),
            ((center_row+21), (center_col-50)),


            ((center_row+17), (center_col-40)),
            ((center_row+17), (center_col-39)),
            ((center_row+17), (center_col-36)),
            ((center_row+17), (center_col-35)),


            ((center_row+17), (center_col-32)),
            ((center_row+17), (center_col-31)),
            ((center_row+17), (center_col-30)),
            ((center_row+17), (center_col-29)),

            ((center_row+17), (center_col-26)),
            ((center_row+17), (center_col-25)),
            ((center_row+17), (center_col-24)),

            ((center_row+17), (center_col-21)),
            ((center_row+17), (center_col-20)),

            ((center_row+17), (center_col-17)),
            ((center_row+17), (center_col-16)),

            ((center_row+17), (center_col-12)),
            ((center_row+17), (center_col-11)),
            ((center_row+17), (center_col-9)),
            ((center_row+17), (center_col-8)),


            ((center_row+16), (center_col-40)),
            ((center_row+16), (center_col-39)),
            ((center_row+16), (center_col-36)),
            ((center_row+16), (center_col-35)),
            ((center_row+16), (center_col-34)),

            ((center_row+16), (center_col-32)),
            ((center_row+16), (center_col-31)),
            ((center_row+16), (center_col-29)),
            ((center_row+16), (center_col-28)),

            ((center_row+16), (center_col-26)),
            ((center_row+16), (center_col-25)),

            ((center_row+16), (center_col-21)),
            ((center_row+16), (center_col-20)),

            ((center_row+16), (center_col-17)),
            ((center_row+16), (center_col-16)),
            ((center_row+16), (center_col-15)),

            ((center_row+16), (center_col-13)),
            ((center_row+16), (center_col-12)),
            ((center_row+16), (center_col-11)),
            ((center_row+16), (center_col-10)),

            ((center_row+16), (center_col-8)),
            ((center_row+16), (center_col-7)),
            ((center_row+16), (center_col-6)),


            ((center_row+15), (center_col-40)),
            ((center_row+15), (center_col-39)),
            ((center_row+15), (center_col-36)),
            ((center_row+15), (center_col-35)),


            ((center_row+15), (center_col-32)),
            ((center_row+15), (center_col-31)),

            ((center_row+15), (center_col-26)),
            ((center_row+15), (center_col-25)),
 
            ((center_row+15), (center_col-21)),
            ((center_row+15), (center_col-20)),

            ((center_row+15), (center_col-17)),
            ((center_row+15), (center_col-16)),

            ((center_row+15), (center_col-12)),
            ((center_row+15), (center_col-11)),
            ((center_row+15), (center_col-8)),
            ((center_row+15), (center_col-7)),

            ((center_row+14), (center_col-41)),
            ((center_row+14), (center_col-40)),
            ((center_row+14), (center_col-39)),
            ((center_row+14), (center_col-36)),
            ((center_row+14), (center_col-35)),

            ((center_row+14), (center_col-32)),
            ((center_row+14), (center_col-31)),
            ((center_row+14), (center_col-30)),
            ((center_row+14), (center_col-29)),
            ((center_row+14), (center_col-28)),

            ((center_row+14), (center_col-26)),
            ((center_row+14), (center_col-25)),

            ((center_row+14), (center_col-22)),
            ((center_row+14), (center_col-21)),
            ((center_row+14), (center_col-20)),
            ((center_row+14), (center_col-17)),
            ((center_row+14), (center_col-16)),

            ((center_row+14), (center_col-12)),
            ((center_row+14), (center_col-11)),
            ((center_row+14), (center_col-8)),
            ((center_row+14), (center_col-7)),


            ((center_row+13), (center_col-40)),
            ((center_row+13), (center_col-39)),
            ((center_row+13), (center_col-36)),
            ((center_row+13), (center_col-35)),

            ((center_row+13), (center_col-29)),
            ((center_row+13), (center_col-28)),

            ((center_row+13), (center_col-26)),
            ((center_row+13), (center_col-25)),

            ((center_row+13), (center_col-21)),
            ((center_row+13), (center_col-20)),
            ((center_row+13), (center_col-17)),
            ((center_row+13), (center_col-16)),

            ((center_row+13), (center_col-12)),
            ((center_row+13), (center_col-11)),
            ((center_row+13), (center_col-8)),
            ((center_row+13), (center_col-7)),

            ((center_row+12), (center_col-39)),
            ((center_row+12), (center_col-36)),
            ((center_row+12), (center_col-35)),

            ((center_row+12), (center_col-32)),
            ((center_row+12), (center_col-29)),
            ((center_row+12), (center_col-28)),

            ((center_row+12), (center_col-26)),
            ((center_row+12), (center_col-25)),
            ((center_row+12), (center_col-23)),

            ((center_row+12), (center_col-20)),
            ((center_row+12), (center_col-17)),
            ((center_row+12), (center_col-16)),

            ((center_row+12), (center_col-12)),
            ((center_row+12), (center_col-11)),
            ((center_row+12), (center_col-10)),
            ((center_row+12), (center_col-8)),
            ((center_row+12), (center_col-7)),
            ((center_row+12), (center_col-6)),


            ((center_row+11), (center_col-38)),
            ((center_row+11), (center_col-37)),

            ((center_row+11), (center_col-33)),
            ((center_row+11), (center_col-31)),
            ((center_row+11), (center_col-29)),
            ((center_row+11), (center_col-28)),

            ((center_row+11), (center_col-26)),
            ((center_row+11), (center_col-25)),
            ((center_row+11), (center_col-24)),

            ((center_row+11), (center_col-19)),
            ((center_row+11), (center_col-18)),

            ((center_row+11), (center_col-12)),
            ((center_row+11), (center_col-11)),
            ((center_row+11), (center_col-8)),
            ((center_row+11), (center_col-7)),


            ((center_row+10), (center_col-33)),
            ((center_row+10), (center_col-30)),
            ((center_row+10), (center_col-29)),
            ((center_row+10), (center_col-28)),

            ((center_row+10), (center_col-26)),


            ((center_row+18), (center_col-38)),
            ((center_row+18), (center_col-37)),

            ((center_row+18), (center_col-26)),
            ((center_row+18), (center_col-25)),
            ((center_row+19), (center_col-25)),

            ((center_row+18), (center_col-19)),
            ((center_row+18), (center_col-18)),



            ((center_row), (center_col + 5)),
            ((center_row), (center_col + 6)),
            ((center_row), (center_col + 8)),
            ((center_row), (center_col + 11)),

            ((center_row-1), (center_col + 4)),
            ((center_row-1), (center_col + 5)),
            ((center_row-1), (center_col + 7)),
            ((center_row-1), (center_col + 9)),
            ((center_row-1), (center_col + 10)),


            ((center_row-2), (center_col + 3)),
            ((center_row-2), (center_col + 4)),
            ((center_row-2), (center_col + 7)),


            ((center_row-3), (center_col + 3)),
            ((center_row-3), (center_col + 4)),
            ((center_row-3), (center_col + 7)),

            ((center_row-4), (center_col + 3)),
            ((center_row-4), (center_col + 4)),
            ((center_row-4), (center_col + 7)),

            ((center_row-5), (center_col + 3)),
            ((center_row-5), (center_col + 4)),
            ((center_row-5), (center_col + 7)),

            ((center_row-6), (center_col + 3)),
            ((center_row-6), (center_col + 4)),
            ((center_row-6), (center_col + 7)),


            ((center_row-7), (center_col + 3)),
            ((center_row-7), (center_col + 4)),
            ((center_row-7), (center_col + 7)),


            ((center_row-8), (center_col + 3)),
            ((center_row-8), (center_col + 4)),
            ((center_row-8), (center_col + 8)),
            ((center_row-8), (center_col + 10)),

            ((center_row-9), (center_col + 4)),
            ((center_row-9), (center_col + 5)),
            ((center_row-9), (center_col + 9)),


            ((center_row-10), (center_col + 5)),
            ((center_row-10), (center_col + 6)),
            ((center_row-10), (center_col + 7)),
            ((center_row-10), (center_col + 8)),



            ((center_row-10), (center_col + 15)),
            ((center_row-10), (center_col + 16)),


            ((center_row-9), (center_col + 14)),
            ((center_row-9), (center_col + 17)),
            ((center_row-9), (center_col + 18)),

            ((center_row-8), (center_col + 13)),
            ((center_row-8), (center_col + 14)),
            ((center_row-8), (center_col + 17)),
            ((center_row-8), (center_col + 18)),

            ((center_row-7), (center_col + 12)),
            ((center_row-7), (center_col + 13)),
            ((center_row-7), (center_col + 14)),
            ((center_row-7), (center_col + 17)),
            ((center_row-7), (center_col + 18)),

            ((center_row-6), (center_col + 13)),
            ((center_row-6), (center_col + 14)),
            ((center_row-6), (center_col + 17)),
            ((center_row-6), (center_col + 18)),


            ((center_row-5), (center_col + 13)),
            ((center_row-5), (center_col + 14)),
            ((center_row-5), (center_col + 17)),
            ((center_row-5), (center_col + 18)),
            ((center_row-5), (center_col + 19)),

            ((center_row-4), (center_col + 13)),
            ((center_row-4), (center_col + 14)),
            ((center_row-4), (center_col + 17)),
            ((center_row-4), (center_col + 18)),

            ((center_row-3), (center_col + 15)),
            ((center_row-3), (center_col + 16)),


            ((center_row-10), (center_col + 21)),
            ((center_row-10), (center_col + 22)),

            ((center_row-9), (center_col + 21)),
            ((center_row-9), (center_col + 22)),
            ((center_row-9), (center_col + 23)),

            ((center_row-8), (center_col + 21)),
            ((center_row-8), (center_col + 22)),


            ((center_row-7), (center_col + 21)),
            ((center_row-7), (center_col + 22)),

            ((center_row-6), (center_col + 21)),
            ((center_row-6), (center_col + 22)),

            ((center_row-5), (center_col + 21)),
            ((center_row-5), (center_col + 22)),

            ((center_row-4), (center_col + 21)),
            ((center_row-4), (center_col + 22)),

            ((center_row-3), (center_col + 21)),
            ((center_row-3), (center_col + 22)),

            ((center_row-2), (center_col + 21)),
            ((center_row-2), (center_col + 22)),

            ((center_row-1), (center_col + 22)),
            ((center_row), (center_col + 23)),


            ((center_row-10), (center_col + 25)),
            ((center_row-10), (center_col + 26)),

            ((center_row-9), (center_col + 25)),
            ((center_row-9), (center_col + 26)),
            ((center_row-9), (center_col + 27)),

            ((center_row-8), (center_col + 25)),
            ((center_row-8), (center_col + 26)),


            ((center_row-7), (center_col + 25)),
            ((center_row-7), (center_col + 26)),

            ((center_row-6), (center_col + 25)),
            ((center_row-6), (center_col + 26)),

            ((center_row-5), (center_col + 25)),
            ((center_row-5), (center_col + 26)),

            ((center_row-4), (center_col + 25)),
            ((center_row-4), (center_col + 26)),

            ((center_row-3), (center_col + 25)),
            ((center_row-3), (center_col + 26)),

            ((center_row-2), (center_col + 25)),
            ((center_row-2), (center_col + 26)),

            ((center_row-1), (center_col + 26)),
            ((center_row), (center_col + 27)),


            ((center_row-10), (center_col + 30)),
            ((center_row-10), (center_col + 31)),
            ((center_row-10), (center_col + 32)),
            ((center_row-10), (center_col + 33)),
            ((center_row-10), (center_col + 38)),
            ((center_row-10), (center_col + 40)),
            ((center_row-10), (center_col + 41)),
            ((center_row-10), (center_col + 45)),
            ((center_row-10), (center_col + 46)),
            ((center_row-10), (center_col + 47)),
            ((center_row-10), (center_col + 48)),



            ((center_row-11), (center_col + 40)),
            ((center_row-11), (center_col + 41)),
            ((center_row-12), (center_col + 40)),
            ((center_row-12), (center_col + 41)),
            ((center_row-12), (center_col + 39)),
            ((center_row-12), (center_col + 38)),
            ((center_row-13), (center_col + 40)),
            ((center_row-13), (center_col + 37)),
            ((center_row-14), (center_col + 36)),



            ((center_row-9), (center_col + 30)),
            ((center_row-9), (center_col + 31)),
            ((center_row-9), (center_col + 34)),
            ((center_row-9), (center_col + 36)),
            ((center_row-9), (center_col + 37)),
            ((center_row-9), (center_col + 39)),
            ((center_row-9), (center_col + 40)),
            ((center_row-9), (center_col + 41)),
            ((center_row-9), (center_col + 44)),
            ((center_row-9), (center_col + 45)),
            ((center_row-9), (center_col + 49)),


            ((center_row-8), (center_col + 30)),
            ((center_row-8), (center_col + 31)),
            ((center_row-8), (center_col + 36)),
            ((center_row-8), (center_col + 37)),
            ((center_row-8), (center_col + 40)),
            ((center_row-8), (center_col + 41)),
            ((center_row-8), (center_col + 44)),
            ((center_row-8), (center_col + 45)),

            ((center_row-7), (center_col + 30)),
            ((center_row-7), (center_col + 31)),
            ((center_row-7), (center_col + 32)),
            ((center_row-7), (center_col + 36)),
            ((center_row-7), (center_col + 37)),
            ((center_row-7), (center_col + 40)),
            ((center_row-7), (center_col + 41)),
            ((center_row-7), (center_col + 44)),
            ((center_row-7), (center_col + 45)),
            ((center_row-7), (center_col + 46)),


            ((center_row-6), (center_col + 30)),
            ((center_row-6), (center_col + 31)),
            ((center_row-6), (center_col + 33)),
            ((center_row-6), (center_col + 36)),
            ((center_row-6), (center_col + 37)),
            ((center_row-6), (center_col + 40)),
            ((center_row-6), (center_col + 41)),
            ((center_row-6), (center_col + 44)),
            ((center_row-6), (center_col + 45)),
            ((center_row-6), (center_col + 47)),


            ((center_row-5), (center_col + 30)),
            ((center_row-5), (center_col + 31)),
            ((center_row-5), (center_col + 33)),
            ((center_row-5), (center_col + 34)),
            ((center_row-5), (center_col + 36)),
            ((center_row-5), (center_col + 37)),
            ((center_row-5), (center_col + 40)),
            ((center_row-5), (center_col + 41)),
            ((center_row-5), (center_col + 44)),
            ((center_row-5), (center_col + 45)),
            ((center_row-5), (center_col + 47)),
            ((center_row-5), (center_col + 48)),


            ((center_row-4), (center_col + 32)),
            ((center_row-4), (center_col + 33)),
            ((center_row-4), (center_col + 38)),
            ((center_row-4), (center_col + 39)),
            ((center_row-4), (center_col + 44)),
            ((center_row-4), (center_col + 46)),
            ((center_row-4), (center_col + 47)),


        ];

        for (row, col) in r_coords.iter() {
            if let Some(cell) = self.current_cells.get_mut(*row, *col) {
                *cell = true;
            }
        }
    }


}

