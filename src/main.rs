#![feature(gen_blocks)]
mod printer;

use std::{array, collections::HashSet, num::NonZero, time::Instant};

use crate::printer::print_board;

fn main() {
    #[rustfmt::skip]
    let example: [Option<NonZero<u8>>; 81] = [
        0,0,0,0,0,0,0,1,0,
        4,0,0,0,0,0,0,0,0,
        0,2,0,0,0,0,0,0,0,
        0,0,0,0,5,0,4,0,7,
        0,0,8,0,0,0,3,0,0,
        0,0,1,0,9,0,0,0,0,
        3,0,0,4,0,0,2,0,0,
        0,5,0,1,0,0,0,0,0,
        0,0,0,8,0,6,0,0,0
    ].map(|v| match v {
        0 => None,
        a => Some(non_zero(a)),
    });

    let mut sudoku = Sudoku::new(example);

    println!("Clues: {}", sudoku.clues_count());
    let time = Instant::now();
    let Some(_) = sudoku.solve() else {
        println!("INVALID");
        return;
    };
    println!("Completed in {} seconds!", time.elapsed().as_secs_f32());
    println!("VALID: {}!", sudoku.is_complete_and_valid());
    print_board(&sudoku)
}

struct Sudoku {
    cells: [Option<NonZero<u8>>; 81],
    annotations: [Vec<NonZero<u8>>; 81],
}

impl Sudoku {
    fn new(cells: [Option<NonZero<u8>>; 81]) -> Self {
        Self {
            annotations: array::repeat::<Vec<NonZero<u8>>, 81>(Vec::new()),
            cells,
        }
    }

    fn clues_count(&self) -> usize {
        self.cells.iter().filter(|c| c.is_some()).count()
    }

    fn is_complete_and_valid(&self) -> bool {
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

    fn solve(&mut self) -> Option<()> {
        self.annotations = array::from_fn(|i| calculate_cell_annotations(&self.cells, i));
        let original_annotations = self.annotations.clone();
        let playable_indices = self.calculate_playable_indices_order();

        let mut playable_index = 0;
        loop {
            let index = playable_indices[playable_index];
            let annotation = &mut self.annotations[index];

            let Some(value_to_try) = annotation.pop() else {
                *annotation = original_annotations[index].clone();
                self.cells[index] = None;
                playable_index = playable_index.checked_sub(1)?;
                continue;
            };

            self.cells[index] = Some(value_to_try);
            let row_i = index / 9;
            let col_i = index % 9;
            let block_i = CELL_TO_BLOCK[index];

            if self.is_valid_block(block_i)
                && self.is_valid_column(col_i)
                && self.is_valid_row(row_i)
            {
                playable_index += 1;
                if playable_index == playable_indices.len() {
                    break;
                }
            }
        }

        Some(())
    }

    // Sorted by annotations length, smaller annotations means it's more probable to hit the right one and not backtrack
    // Note: Sorting has the biggest impact, it goes from 2 seconds to 0 seconds to solve a 17 clues sudoku we used as example
    fn calculate_playable_indices_order(&mut self) -> Vec<usize> {
        let mut playable_indices = self
            .annotations
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

const fn non_zero(value: u8) -> NonZero<u8> {
    match NonZero::new(value) {
        Some(v) => v,
        None => unreachable!(),
    }
}

fn calculate_cell_annotations(
    cells: &[Option<NonZero<u8>>; 81],
    cell_i: usize,
) -> Vec<NonZero<u8>> {
    if cells[cell_i].is_some() {
        return vec![];
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
        .collect::<Vec<_>>()
}

fn full_annotations() -> Vec<NonZero<u8>> {
    vec![
        non_zero(1),
        non_zero(2),
        non_zero(3),
        non_zero(4),
        non_zero(5),
        non_zero(6),
        non_zero(7),
        non_zero(8),
        non_zero(9),
    ]
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
