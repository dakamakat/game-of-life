use core::fmt;
use std::{
    fmt::Display,
    process::{ExitCode, Termination},
};

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{coordinate::Coordinate, pattern::init_pattern};

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(pattern: u32 , width: u32 , height: u32) -> Universe {
        let size = (width * height) as usize;

        let mut cells = FixedBitSet::with_capacity(size);

        init_pattern(pattern, &mut cells, size);

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn set_width(&mut self, width: u32) -> u32 {
        self.width = width;
        let capacity = (self.height * width) as usize;
        FixedBitSet::with_capacity(capacity);
        width
    }

    pub fn set_height(&mut self, height: u32) -> u32 {
        self.height = height;
        let capacity = (self.width * height) as usize;
        FixedBitSet::with_capacity(capacity);
        height
    }

    pub fn set_cells(&mut self, cells: Vec<Coordinate>) {
        for coordinate in cells.iter().cloned() {
            let idx = self.get_index(coordinate.y, coordinate.x);
            self.cells.set(idx, true)
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (true, x) if x < 2 => false,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (true, 2) | (true, 3) => true,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (true, x) if x > 3 => false,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (false, 3) => true,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const usize {
        self.cells.as_slice().as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
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
}

/// Non wasm Impl for testing purposes
impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells_borrow(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true)
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];

                let symbol = if cell { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
impl Termination for Universe {
    fn report(self) -> std::process::ExitCode {
        ExitCode::SUCCESS
    }
}
