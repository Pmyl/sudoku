use std::{
    array,
    collections::{HashSet, VecDeque},
    num::NonZero,
};

pub struct Sudoku {
    pub cells: [Option<NonZero<u8>>; 81],
    pub original_cells: [Option<NonZero<u8>>; 81],
    pub annotations: [VecDeque<NonZero<u8>>; 81],
    pub original_annotations: [VecDeque<NonZero<u8>>; 81],
    pub playable_indices: Vec<usize>,
    pub playable_index: usize,
    pub step_count: usize,
    pub is_valid: bool,
    pub backtracking_count: usize,
    pub invalid_attempts: usize,
    pub visited_cells_heatmap: [usize; 81],
    pub invalid_attempts_heatmap: [usize; 81],
    pub backtracked_heatmap: [usize; 81],
}

impl Sudoku {
    pub fn new(cells: [Option<NonZero<u8>>; 81]) -> Self {
        let annotations = array::from_fn(|i| calculate_cell_annotations(&cells, i));
        let original_annotations = annotations.clone();
        let playable_indices = calculate_playable_indices_order(&annotations);

        Self {
            original_cells: cells.clone(),
            annotations,
            original_annotations,
            playable_indices,
            cells,
            playable_index: 0,
            step_count: 0,
            is_valid: true,
            backtracking_count: 0,
            invalid_attempts: 0,
            visited_cells_heatmap: [0; _],
            invalid_attempts_heatmap: [0; _],
            backtracked_heatmap: [0; _],
        }
    }

    pub fn reset(&mut self) {
        self.annotations = self.original_annotations.clone();
        self.playable_indices = calculate_playable_indices_order(&self.annotations);
        self.playable_index = 0;
        self.cells = self.original_cells.clone();
        self.step_count = 0;
        self.is_valid = true;
        self.backtracking_count = 0;
        self.invalid_attempts = 0;
        self.visited_cells_heatmap = [0; _];
        self.invalid_attempts_heatmap = [0; _];
        self.backtracked_heatmap = [0; _];
    }

    pub fn index(&self) -> Option<usize> {
        self.playable_indices.get(self.playable_index).cloned()
    }

    pub fn clues_count(&self) -> usize {
        self.cells.iter().filter(|c| c.is_some()).count()
    }

    pub fn is_complete_and_valid(&self) -> bool {
        // columns
        for col_i in 0..9 {
            let mut set = HashSet::<NonZero<u8>>::new();
            for cell_i in 0..9 {
                let Some(cell) = &self.cells[col_i + cell_i * 9] else {
                    return false;
                };
                set.insert(*cell);
            }
            let how_many = set.len();
            if how_many != 9 {
                return false;
            }
        }

        // rows
        for row in self.cells.chunks_exact(9) {
            let how_many = row
                .into_iter()
                .cloned()
                .filter_map(|r| r)
                .collect::<HashSet<NonZero<u8>>>()
                .len();
            if how_many != 9 {
                return false;
            }
        }

        // blocks
        for block_i in 0usize..9 {
            let mut block: [&Option<NonZero<u8>>; 9] = [&None; 9];
            let start_i = (block_i / 3) * 27 + (block_i.rem_euclid(3) * 3);
            for (chunk_i, cells) in [0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                .chunks_exact(3)
                .enumerate()
            {
                for block_cell_i in cells {
                    let i = start_i + block_cell_i + chunk_i * 6;
                    block[*block_cell_i] = &self.cells[i];
                }
            }

            let how_many = block
                .into_iter()
                .cloned()
                .filter_map(|r| r)
                .collect::<HashSet<NonZero<u8>>>()
                .len();
            if how_many != 9 {
                return false;
            }
        }

        true
    }

    // Note: Using `set` as an array instead of a HashSet gives a big impact in performance (from 40~ seconds to 4~ seconds to solve a 17 clues sudoku)
    fn is_valid_column(&self, col_i: usize) -> bool {
        let column = column_cells(&self.cells, col_i);

        let mut set = [false; 10];

        for v in column {
            let Some(value) = v else {
                continue;
            };

            if set[value.get() as usize] {
                return false;
            }

            set[value.get() as usize] = true;
        }

        return true;
    }

    // Note: Using `set` as an array instead of a HashSet gives a big impact in performance (from 40~ seconds to 4~ seconds to solve a 17 clues sudoku)
    fn is_valid_row(&self, row_i: usize) -> bool {
        let row = row_cells(&self.cells, row_i);

        let mut set = [false; 10];

        for cell_i in 0..9 {
            let Some(value) = &row[cell_i] else {
                continue;
            };

            if set[value.get() as usize] {
                return false;
            }

            set[value.get() as usize] = true;
        }

        return true;
    }

    // Note: Using `set` as an array instead of a HashSet gives a big impact in performance (from 40~ seconds to 4~ seconds to solve a 17 clues sudoku)
    fn is_valid_block(&self, block_i: usize) -> bool {
        let block_indices = &BLOCK_TO_INDICES[block_i];

        let mut set = [false; 10];

        for cell_i in block_indices {
            let Some(value) = &self.cells[*cell_i] else {
                continue;
            };

            if set[value.get() as usize] {
                return false;
            }

            set[value.get() as usize] = true;
        }

        return true;
    }

    pub fn solve(&mut self) -> Option<()> {
        loop {
            match self.next() {
                Some(true) => return Some(()),
                Some(false) => return None,
                None => (),
            };
        }
    }

    pub fn next(&mut self) -> Option<bool> {
        self.step_count += 1;
        let index = self.playable_indices[self.playable_index];
        self.visited_cells_heatmap[index] += 1;
        let annotation = &mut self.annotations[index];

        let Some(value_to_try) = annotation.pop_front() else {
            *annotation = self.original_annotations[index].clone();
            self.cells[index] = None;
            self.backtracking_count += 1;
            self.backtracked_heatmap[index] += 1;
            match self.playable_index.checked_sub(1) {
                Some(v) => {
                    self.playable_index = v;
                    return None;
                }
                None => return Some(false),
            }
        };

        self.cells[index] = Some(value_to_try);
        let row_i = index / 9;
        let col_i = index % 9;
        let block_i = CELL_TO_BLOCK[index];

        if self.is_valid_block(block_i) && self.is_valid_column(col_i) && self.is_valid_row(row_i) {
            self.is_valid = true;
            self.playable_index += 1;
            if self.playable_index == self.playable_indices.len() {
                return Some(true);
            }
        } else {
            self.invalid_attempts += 1;
            self.invalid_attempts_heatmap[index] += 1;
            self.is_valid = false;
        }

        return None;
    }
}

// Sorted by annotations length, smaller annotations means it's more probable to hit the right one and not backtrack
// Note: Sorting has the biggest impact, it goes from 2 seconds to 0 seconds to solve a 17 clues sudoku we used as example
fn calculate_playable_indices_order(annotations: &[VecDeque<NonZero<u8>>; 81]) -> Vec<usize> {
    let mut playable_indices = annotations
        .iter()
        .enumerate()
        .filter(|(_, a)| !a.is_empty())
        .map(|(i, a)| (i, a.len()))
        .collect::<Vec<_>>();
    playable_indices.sort_by_key(|(_, l)| *l);
    let playable_indices = playable_indices
        .into_iter()
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    playable_indices
}

#[rustfmt::skip]
const CELL_TO_BLOCK: [usize; 81] = [
    0, 0, 0, 1, 1, 1, 2, 2, 2,
    0, 0, 0, 1, 1, 1, 2, 2, 2,
    0, 0, 0, 1, 1, 1, 2, 2, 2,
    3, 3, 3, 4, 4, 4, 5, 5, 5,
    3, 3, 3, 4, 4, 4, 5, 5, 5,
    3, 3, 3, 4, 4, 4, 5, 5, 5,
    6, 6, 6, 7, 7, 7, 8, 8, 8,
    6, 6, 6, 7, 7, 7, 8, 8, 8,
    6, 6, 6, 7, 7, 7, 8, 8, 8,
];

#[rustfmt::skip]
const CELL_TO_ROW: [usize; 81] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 5, 5, 5, 5, 5,
    6, 6, 6, 6, 6, 6, 6, 6, 6,
    7, 7, 7, 7, 7, 7, 7, 7, 7,
    8, 8, 8, 8, 8, 8, 8, 8, 8,
];

#[rustfmt::skip]
const CELL_TO_COLUMN: [usize; 81] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
];

#[rustfmt::skip]
const BLOCK_TO_INDICES: [[usize; 9]; 9] = [
    [ 0,  1,  2,  9, 10, 11, 18, 19, 20],
    [ 3,  4,  5, 12, 13, 14, 21, 22, 23],
    [ 6,  7,  8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

fn calculate_cell_annotations(
    cells: &[Option<NonZero<u8>>; 81],
    cell_i: usize,
) -> VecDeque<NonZero<u8>> {
    if cells[cell_i].is_some() {
        return VecDeque::new();
    }

    let mut available_as_set = [true; 9]; // value 1 is index 0 and so on, if true it's available, otherwise false

    // check row
    let row = row_cells(cells, CELL_TO_ROW[cell_i]);
    row.iter().for_each(|value| match value {
        Some(value) => available_as_set[value.get() as usize - 1] = false,
        None => {}
    });

    // check column
    let column = column_cells(cells, CELL_TO_COLUMN[cell_i]);
    column.for_each(|value| match value {
        Some(value) => available_as_set[value.get() as usize - 1] = false,
        None => {}
    });

    // check block
    let block = block_cells(cells, CELL_TO_BLOCK[cell_i]);
    block.for_each(|value| match value {
        Some(value) => available_as_set[value.get() as usize - 1] = false,
        None => {}
    });

    available_as_set
        .into_iter()
        .enumerate()
        .filter_map(|(i, present)| NonZero::new(if present { i as u8 + 1 } else { 0 }))
        .collect::<VecDeque<_>>()
}

fn row_cells(cells: &[Option<NonZero<u8>>; 81], row_i: usize) -> &[Option<NonZero<u8>>] {
    &cells[row_i * 9..row_i * 9 + 9]
}

fn column_cells(
    cells: &[Option<NonZero<u8>>; 81],
    col_i: usize,
) -> impl Iterator<Item = &Option<NonZero<u8>>> {
    gen move {
        for cell_i in 0..9 {
            yield &cells[col_i + cell_i * 9];
        }
    }
}

fn block_cells(
    cells: &[Option<NonZero<u8>>; 81],
    block_i: usize,
) -> impl Iterator<Item = &Option<NonZero<u8>>> {
    gen move {
        for cell_i in BLOCK_TO_INDICES[block_i] {
            yield &cells[cell_i];
        }
    }
}
