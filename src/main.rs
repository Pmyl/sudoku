mod printer;

use std::{array, collections::HashSet, num::NonZero};

use crate::printer::print_board;

fn main() {
    let mut sudoku = Sudoku::new(
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
        .map(|v| match v {
            0 => None,
            a => Some(non_zero(a)),
        }),
    );

    let Some(_) = sudoku.solve() else {
        println!("INVALID");
        return;
    };
    println!("Is complete and valid {}", sudoku.is_complete_and_valid());
    print_board(&sudoku)
}

struct Sudoku {
    cells: [Option<NonZero<u8>>; 81],
    clues_indices: Vec<usize>,
    annotations: [Vec<NonZero<u8>>; 81],
}

impl Sudoku {
    fn new(cells: [Option<NonZero<u8>>; 81]) -> Self {
        Self {
            annotations: array::repeat::<Vec<NonZero<u8>>, 81>(Vec::new()),
            clues_indices: cells
                .iter()
                .enumerate()
                .filter_map(|(i, c)| c.map(|_| i))
                .collect::<Vec<usize>>(),
            cells,
        }
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

    fn is_valid_column(&self, col_i: usize) -> bool {
        let mut set = HashSet::<NonZero<u8>>::new();
        for cell_i in 0..9 {
            let Some(cell) = &self.cells[col_i + cell_i * 9] else {
                continue;
            };

            if !set.insert(*cell) {
                return false;
            }
        }

        return true;
    }

    fn is_valid_row(&self, row_i: usize) -> bool {
        let row = &self.cells[row_i * 9..row_i * 9 + 9];

        let mut set = HashSet::<NonZero<u8>>::new();

        for cell_i in 0..9 {
            let Some(cell) = &row[cell_i] else {
                continue;
            };

            if !set.insert(*cell) {
                return false;
            }
        }

        return true;
    }

    fn is_valid_block(&self, block_i: usize) -> bool {
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

        let mut set = HashSet::<NonZero<u8>>::new();

        for cell_i in 0..9 {
            let Some(cell) = &block[cell_i] else {
                continue;
            };

            if !set.insert(*cell) {
                return false;
            }
        }

        return true;
    }

    fn solve(&mut self) -> Option<()> {
        self.annotations = array::repeat::<Vec<NonZero<u8>>, 81>(full_annotations());

        let mut index = 0;
        while self.clues_indices.contains(&index) {
            index += 1;
        }
        loop {
            let annotation = &mut self.annotations[index];

            let Some(value_to_try) = annotation.pop() else {
                *annotation = full_annotations();
                self.cells[index] = None;
                index = index.checked_sub(1)?;
                while self.clues_indices.contains(&index) {
                    index = index.checked_sub(1)?;
                }
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
                index += 1;
                while self.clues_indices.contains(&index) {
                    index += 1;
                }

                if index == 81 {
                    break;
                }
            }
        }

        Some(())
    }
}

fn non_zero(value: u8) -> NonZero<u8> {
    match NonZero::new(value) {
        Some(v) => v,
        None => unreachable!(),
    }
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
