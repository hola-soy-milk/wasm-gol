mod utils;

use std::fmt;

use wasm_bindgen::prelude::*;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
extern crate js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {

    pub fn add_pulsar(&mut self, row: u32, column: u32) {
        if ( row - 6 )  > 0 && ( column - 6 ) > 0 && ( row + 6 ) < self.height - 1 && ( column + 6 ) < self.width - 1 {
            self.cells.set(self.get_index(row+6, column-6), false);
            self.cells.set(self.get_index(row+6, column-5), false);
            self.cells.set(self.get_index(row+6, column-4), true);
            self.cells.set(self.get_index(row+6, column-3), true);
            self.cells.set(self.get_index(row+6, column-2), true);
            self.cells.set(self.get_index(row+6, column-1), false);
            self.cells.set(self.get_index(row+6, column), false);
            self.cells.set(self.get_index(row+6, column+1), false);
            self.cells.set(self.get_index(row+6, column+2), true);
            self.cells.set(self.get_index(row+6, column+3), true);
            self.cells.set(self.get_index(row+6, column+4), true);
            self.cells.set(self.get_index(row+6, column+5), false);
            self.cells.set(self.get_index(row+6, column+6), false);

            self.cells.set(self.get_index(row+5, column-6), false);
            self.cells.set(self.get_index(row+5, column-5), false);
            self.cells.set(self.get_index(row+5, column-4), false);
            self.cells.set(self.get_index(row+5, column-3), false);
            self.cells.set(self.get_index(row+5, column-2), false);
            self.cells.set(self.get_index(row+5, column-1), false);
            self.cells.set(self.get_index(row+5, column), false);
            self.cells.set(self.get_index(row+5, column+1), false);
            self.cells.set(self.get_index(row+5, column+2), false);
            self.cells.set(self.get_index(row+5, column+3), false);
            self.cells.set(self.get_index(row+5, column+4), false);
            self.cells.set(self.get_index(row+5, column+5), false);
            self.cells.set(self.get_index(row+5, column+6), false);

            self.cells.set(self.get_index(row+4, column-6), true);
            self.cells.set(self.get_index(row+4, column-5), false);
            self.cells.set(self.get_index(row+4, column-4), false);
            self.cells.set(self.get_index(row+4, column-3), false);
            self.cells.set(self.get_index(row+4, column-2), false);
            self.cells.set(self.get_index(row+4, column-1), true);
            self.cells.set(self.get_index(row+4, column), false);
            self.cells.set(self.get_index(row+4, column+1), true);
            self.cells.set(self.get_index(row+4, column+2), false);
            self.cells.set(self.get_index(row+4, column+3), false);
            self.cells.set(self.get_index(row+4, column+4), false);
            self.cells.set(self.get_index(row+4, column+5), false);
            self.cells.set(self.get_index(row+4, column+6), true);

            self.cells.set(self.get_index(row+3, column-6), true);
            self.cells.set(self.get_index(row+3, column-5), false);
            self.cells.set(self.get_index(row+3, column-4), false);
            self.cells.set(self.get_index(row+3, column-3), false);
            self.cells.set(self.get_index(row+3, column-2), false);
            self.cells.set(self.get_index(row+3, column-1), true);
            self.cells.set(self.get_index(row+3, column), false);
            self.cells.set(self.get_index(row+3, column+1), true);
            self.cells.set(self.get_index(row+3, column+2), false);
            self.cells.set(self.get_index(row+3, column+3), false);
            self.cells.set(self.get_index(row+3, column+4), false);
            self.cells.set(self.get_index(row+3, column+5), false);
            self.cells.set(self.get_index(row+3, column+6), true);

            self.cells.set(self.get_index(row+2, column-6), true);
            self.cells.set(self.get_index(row+2, column-5), false);
            self.cells.set(self.get_index(row+2, column-4), false);
            self.cells.set(self.get_index(row+2, column-3), false);
            self.cells.set(self.get_index(row+2, column-2), false);
            self.cells.set(self.get_index(row+2, column-1), true);
            self.cells.set(self.get_index(row+2, column), false);
            self.cells.set(self.get_index(row+2, column+1), true);
            self.cells.set(self.get_index(row+2, column+2), false);
            self.cells.set(self.get_index(row+2, column+3), false);
            self.cells.set(self.get_index(row+2, column+4), false);
            self.cells.set(self.get_index(row+2, column+5), false);
            self.cells.set(self.get_index(row+2, column+6), true);

            self.cells.set(self.get_index(row+1, column-6), false);
            self.cells.set(self.get_index(row+1, column-5), false);
            self.cells.set(self.get_index(row+1, column-4), true);
            self.cells.set(self.get_index(row+1, column-3), true);
            self.cells.set(self.get_index(row+1, column-2), true);
            self.cells.set(self.get_index(row+1, column-1), false);
            self.cells.set(self.get_index(row+1, column), false);
            self.cells.set(self.get_index(row+1, column+1), false);
            self.cells.set(self.get_index(row+1, column+2), true);
            self.cells.set(self.get_index(row+1, column+3), true);
            self.cells.set(self.get_index(row+1, column+4), true);
            self.cells.set(self.get_index(row+1, column+5), false);
            self.cells.set(self.get_index(row+1, column+6), false);

            self.cells.set(self.get_index(row-6, column-6), false);
            self.cells.set(self.get_index(row-6, column-5), false);
            self.cells.set(self.get_index(row-6, column-4), true);
            self.cells.set(self.get_index(row-6, column-3), true);
            self.cells.set(self.get_index(row-6, column-2), true);
            self.cells.set(self.get_index(row-6, column-1), false);
            self.cells.set(self.get_index(row-6, column), false);
            self.cells.set(self.get_index(row-6, column+1), false);
            self.cells.set(self.get_index(row-6, column+2), true);
            self.cells.set(self.get_index(row-6, column+3), true);
            self.cells.set(self.get_index(row-6, column+4), true);
            self.cells.set(self.get_index(row-6, column+5), false);
            self.cells.set(self.get_index(row-6, column+6), false);

            self.cells.set(self.get_index(row-5, column-6), false);
            self.cells.set(self.get_index(row-5, column-5), false);
            self.cells.set(self.get_index(row-5, column-4), false);
            self.cells.set(self.get_index(row-5, column-3), false);
            self.cells.set(self.get_index(row-5, column-2), false);
            self.cells.set(self.get_index(row-5, column-1), false);
            self.cells.set(self.get_index(row-5, column), false);
            self.cells.set(self.get_index(row-5, column+1), false);
            self.cells.set(self.get_index(row-5, column+2), false);
            self.cells.set(self.get_index(row-5, column+3), false);
            self.cells.set(self.get_index(row-5, column+4), false);
            self.cells.set(self.get_index(row-5, column+5), false);
            self.cells.set(self.get_index(row-5, column+6), false);

            self.cells.set(self.get_index(row-4, column-6), true);
            self.cells.set(self.get_index(row-4, column-5), false);
            self.cells.set(self.get_index(row-4, column-4), false);
            self.cells.set(self.get_index(row-4, column-3), false);
            self.cells.set(self.get_index(row-4, column-2), false);
            self.cells.set(self.get_index(row-4, column-1), true);
            self.cells.set(self.get_index(row-4, column), false);
            self.cells.set(self.get_index(row-4, column+1), true);
            self.cells.set(self.get_index(row-4, column+2), false);
            self.cells.set(self.get_index(row-4, column+3), false);
            self.cells.set(self.get_index(row-4, column+4), false);
            self.cells.set(self.get_index(row-4, column+5), false);
            self.cells.set(self.get_index(row-4, column+6), true);

            self.cells.set(self.get_index(row-3, column-6), true);
            self.cells.set(self.get_index(row-3, column-5), false);
            self.cells.set(self.get_index(row-3, column-4), false);
            self.cells.set(self.get_index(row-3, column-3), false);
            self.cells.set(self.get_index(row-3, column-2), false);
            self.cells.set(self.get_index(row-3, column-1), true);
            self.cells.set(self.get_index(row-3, column), false);
            self.cells.set(self.get_index(row-3, column+1), true);
            self.cells.set(self.get_index(row-3, column+2), false);
            self.cells.set(self.get_index(row-3, column+3), false);
            self.cells.set(self.get_index(row-3, column+4), false);
            self.cells.set(self.get_index(row-3, column+5), false);
            self.cells.set(self.get_index(row-3, column+6), true);

            self.cells.set(self.get_index(row-2, column-6), true);
            self.cells.set(self.get_index(row-2, column-5), false);
            self.cells.set(self.get_index(row-2, column-4), false);
            self.cells.set(self.get_index(row-2, column-3), false);
            self.cells.set(self.get_index(row-2, column-2), false);
            self.cells.set(self.get_index(row-2, column-1), true);
            self.cells.set(self.get_index(row-2, column), false);
            self.cells.set(self.get_index(row-2, column+1), true);
            self.cells.set(self.get_index(row-2, column+2), false);
            self.cells.set(self.get_index(row-2, column+3), false);
            self.cells.set(self.get_index(row-2, column+4), false);
            self.cells.set(self.get_index(row-2, column+5), false);
            self.cells.set(self.get_index(row-2, column+6), true);

            self.cells.set(self.get_index(row-1, column-6), false);
            self.cells.set(self.get_index(row-1, column-5), false);
            self.cells.set(self.get_index(row-1, column-4), true);
            self.cells.set(self.get_index(row-1, column-3), true);
            self.cells.set(self.get_index(row-1, column-2), true);
            self.cells.set(self.get_index(row-1, column-1), false);
            self.cells.set(self.get_index(row-1, column), false);
            self.cells.set(self.get_index(row-1, column+1), false);
            self.cells.set(self.get_index(row-1, column+2), true);
            self.cells.set(self.get_index(row-1, column+3), true);
            self.cells.set(self.get_index(row-1, column+4), true);
            self.cells.set(self.get_index(row-1, column+5), false);
            self.cells.set(self.get_index(row-1, column+6), false);

            self.cells.set(self.get_index(row, column-6), false);
            self.cells.set(self.get_index(row, column-5), false);
            self.cells.set(self.get_index(row, column-4), false);
            self.cells.set(self.get_index(row, column-3), false);
            self.cells.set(self.get_index(row, column-2), false);
            self.cells.set(self.get_index(row, column-1), false);
            self.cells.set(self.get_index(row, column), false);
            self.cells.set(self.get_index(row, column+1), false);
            self.cells.set(self.get_index(row, column+2), false);
            self.cells.set(self.get_index(row, column+3), false);
            self.cells.set(self.get_index(row, column+4), false);
            self.cells.set(self.get_index(row, column+5), false);
            self.cells.set(self.get_index(row, column+6), false);
        }
    }

    pub fn add_glider(&mut self, row: u32, column: u32) {
        if row > 0 && column > 0 && row < self.height - 1 && column < self.width - 1 {
            self.cells.set(self.get_index(row-1, column-1), false);
            self.cells.set(self.get_index(row-1, column), false);
            self.cells.set(self.get_index(row-1, column+1), true);
            self.cells.set(self.get_index(row, column-1), true);
            self.cells.set(self.get_index(row, column), false);
            self.cells.set(self.get_index(row, column+1), true);
            self.cells.set(self.get_index(row+1, column-1), false);
            self.cells.set(self.get_index(row+1, column), true);
            self.cells.set(self.get_index(row+1, column+1), true);
        }
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(idx, match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise
                });
            }
        }

        self.cells = next;
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn reset_cells(&mut self) {
        let size = (self.width * self.height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
        }
        self.cells = cells;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.reset_cells();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.reset_cells();
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;
        let size = (width * height) as usize;
        let cells = FixedBitSet::with_capacity(size);

        let mut universe = Universe {
            width,
            height,
            cells,
        };
        universe.reset_cells();
        universe
    }

    pub fn kill_all_cells(&mut self) {
        let size = (self.width * self.height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, false);
        }
        self.cells = cells;
    }
}


impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}
