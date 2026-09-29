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

    println!("Clues: {}", 81 - sudoku.playable_indices.len());
    let time = Instant::now();
    let Some(_) = sudoku.solve() else {
        println!("INVALID");
        return;
    };
    println!("Completed in {} seconds!", time.elapsed().as_secs_f32());
    print_board(&sudoku)
}

struct Sudoku {
    cells: [Option<NonZero<u8>>; 81],
    playable_indices: Vec<usize>,
    annotations: [Vec<NonZero<u8>>; 81],
}

impl Sudoku {
    fn new(cells: [Option<NonZero<u8>>; 81]) -> Self {
        Self {
            annotations: array::repeat::<Vec<NonZero<u8>>, 81>(Vec::new()),
            playable_indices: cells
                .iter()
                .enumerate()
                .filter_map(|(i, c)| match c {
                    Some(_) => None,
                    None => Some(i),
                })
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

        let mut playable_index = 0;
        loop {
            let index = self.playable_indices[playable_index];
            let annotation = &mut self.annotations[index];

            let Some(value_to_try) = annotation.pop() else {
                *annotation = full_annotations();
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
                if playable_index == self.playable_indices.len() {
                    break;
                }
            }
        }

        Some(())
    }
}

const fn non_zero(value: u8) -> NonZero<u8> {
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
