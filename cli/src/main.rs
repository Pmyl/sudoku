mod printer;

use core::sudoku::Sudoku;
use std::{num::NonZero, time::Instant};

use data::{SUDOKU_EXAMPLE_2, ToNonZero, norwig1, norwig2};

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

    let examples = norwig1();

    let examples2 = norwig2();

    for example in examples2 {
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

const fn non_zero(value: u8) -> NonZero<u8> {
    match NonZero::new(value) {
        Some(v) => v,
        None => unreachable!(),
    }
}

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
