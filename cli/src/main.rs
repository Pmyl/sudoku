mod printer;

use core::sudoku::Sudoku;
use std::{num::NonZero, time::Instant};

use data::{SUDOKU_EXAMPLE_1, SUDOKU_EXAMPLE_2, ToNonZero, norwig1, norwig2};

use crate::printer::print_board;

fn main() {
    let _example1 = SUDOKU_EXAMPLE_1.to_non_zero();

    let _norwig1_examples = norwig1();

    let norwig2_examples = norwig2();

    for example in norwig2_examples {
        solve_sudoku_puzzle(example);
    }

    solve_sudoku_puzzle(SUDOKU_EXAMPLE_2.to_non_zero());
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
