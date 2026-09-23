use std::{collections::HashSet, num::NonZero};

fn main() {
    let sudoku = Sudoku {
        cells: [
            3, 2, 8, 6, 5, 4, 9, 1, 7, 4, 7, 1, 9, 2, 3, 8, 6, 5, 5, 6, 9, 1, 7, 8, 4, 2, 3, 6, 8,
            4, 5, 3, 7, 2, 9, 1, 2, 1, 5, 4, 8, 9, 3, 7, 6, 9, 3, 7, 2, 1, 6, 5, 8, 4, 1, 5, 3, 8,
            6, 2, 7, 4, 9, 7, 9, 2, 3, 4, 1, 6, 5, 8, 8, 4, 6, 7, 9, 5, 1, 3, 2,
        ]
        .map(|n| {
            if n != 0 {
                Some(n.try_into().unwrap())
            } else {
                None
            }
        }),
    };

    println!("Is complete and valid {}", sudoku.is_complete_and_valid());
}

struct Sudoku {
    cells: [Option<NonZero<u8>>; 81],
}

impl Sudoku {
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
}
