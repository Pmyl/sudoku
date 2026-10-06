#![feature(gen_blocks)]
mod printer;

use std::{
    array,
    collections::{HashSet, VecDeque},
    num::NonZero,
    time::Instant,
};

use crate::printer::print_board;

fn main() {
    #[rustfmt::skip]
    let example1: [Option<NonZero<u8>>; 81] = [
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

    #[rustfmt::skip]
    let example2: [Option<NonZero<u8>>; 81] = [
        0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,3,0,8,5,
        0,0,1,0,2,0,0,0,0,
        0,0,0,5,0,7,0,0,0,
        0,0,4,0,0,0,1,0,0,
        0,9,0,0,0,0,0,0,0,
        5,0,0,0,0,0,0,7,3,
        0,0,2,0,1,0,0,0,0,
        0,0,0,0,4,0,0,0,9
    ].map(|v| match v {
        0 => None,
        a => Some(non_zero(a)),
    });

    let examples = norvig_sudoku_reader_puzzles(
        "85...24..72......9..4.........1.7..23.5...9...4...........8..7..17..........36.4.
        ..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..
        12..4......5.69.1...9...5.........7.7...52.9..3......2.9.6...5.4..9..8.1..3...9.4
        ...57..3.1......2.7...234......8...4..7..4...49....6.5.42...3.....7..9....18.....
        7..1523........92....3.....1....47.8.......6............9...5.6.4.9.7...8....6.1.
        1....7.9..3..2...8..96..5....53..9...1..8...26....4...3......1..4......7..7...3..
        1...34.8....8..5....4.6..21.18......3..1.2..6......81.52..7.9....6..9....9.64...2
        ...92......68.3...19..7...623..4.1....1...7....8.3..297...8..91...5.72......64...
        .6.5.4.3.1...9...8.........9...5...6.4.6.2.7.7...4...5.........4...8...1.5.2.3.4.
        7.....4...2..7..8...3..8.799..5..3...6..2..9...1.97..6...3..9...3..4..6...9..1.35
        ....7..2.8.......6.1.2.5...9.54....8.........3....85.1...3.2.8.4.......9.7..6....",
        '.',
    );

    let examples2 = norvig_sudoku_reader_puzzles(
        "000000010400000000020000000000050407008000300001090000300400200050100000000806000
        000000010400000000020000000000050604008000300001090000300400200050100000000807000
        000000012000035000000600070700000300000400800100000000000120000080000040050000600
        000000012003600000000007000410020000000500300700000600280000040000300500000000000
        000000012008030000000000040120500000000004700060000000507000300000620000000100000
        000000012040050000000009000070600400000100000000000050000087500601000300200000000
        000000012050400000000000030700600400001000000000080000920000800000510700000003000
        000000012300000060000040000900000500000001070020000000000350400001400800060000000
        000000012400090000000000050070200000600000400000108000018000000000030700502000000
        000000012500008000000700000600120000700000450000030000030000800000500700020000000
        000000012700060000000000050080200000600000400000109000019000000000030800502000000
        000000012800040000000000060090200000700000400000501000015000000000030900602000000",
        '0',
    );

    // TODO: 000000012300000060000040000900000500000001070020000000000350400001400800060000000
    //       This is the worst one, it took 130 seconds!!!

    for example in examples2 {
        solve_sudoku_puzzle(example);
    }
}

fn solve_sudoku_puzzle(example2: [Option<NonZero<u8>>; 81]) {
    let mut sudoku = Sudoku::new(example2);

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
    annotations: [VecDeque<NonZero<u8>>; 81],
}

impl Sudoku {
    fn new(cells: [Option<NonZero<u8>>; 81]) -> Self {
        Self {
            annotations: array::repeat::<VecDeque<NonZero<u8>>, 81>(VecDeque::new()),
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

            let Some(value_to_try) = annotation.pop_front() else {
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

fn norvig_sudoku_reader_puzzles(puzzles: &str, empty_char: char) -> Vec<[Option<NonZero<u8>>; 81]> {
    puzzles
        .lines()
        .map(|l| l.trim())
        .map(|puzzle| norvig_sudoku_reader(puzzle, empty_char))
        .collect::<Vec<_>>()
}

fn norvig_sudoku_reader(puzzle: &str, empty_char: char) -> [Option<NonZero<u8>>; 81] {
    let mut cells = [None; 81];

    for (i, c) in puzzle.chars().enumerate() {
        let value = if c == empty_char {
            None
        } else {
            Some(NonZero::new(c.to_digit(10).unwrap() as u8).unwrap())
        };
        cells[i] = value;
    }

    cells
}
