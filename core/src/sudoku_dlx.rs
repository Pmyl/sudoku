use std::num::NonZero;

use crate::sudoku::Sudoku;

// Exact cover explanation: https://en.wikipedia.org/wiki/Exact_cover
// Exact cover matrix for Sudoku: https://www.stolaf.edu/people/hansonr/sudoku/exactcovermatrix.htm
// Per ogni cella iniziale (original_cells), dobbiamo togliere dalla exact cover matrix
// tutte le righe di constraint // dei numeri diversi dal numero nella cella.
// Per esempio: se r1c7 ha il numero inizial 5, va tenuta solo la riga
// r1c7#5 e vanno rimosse r1c7#1, r1c7#2, r1c7#3, r1c7#4, r1c7#6, r1c7#7, r1c7#8, r1c7#9
//
// SudokuX usa https://en.wikipedia.org/wiki/Dancing_links
// rispetto a SudokuXNaive

pub struct SudokuDlx {}

impl SudokuDlx {
    pub fn new(_cells: [Option<NonZero<u8>>; 81]) -> SudokuDlx {
        SudokuDlx {}
    }
}

impl Sudoku for SudokuDlx {
    fn reset(&mut self) {
        todo!()
    }

    fn index(&self) -> Option<usize> {
        todo!()
    }

    fn clues_count(&self) -> usize {
        todo!()
    }

    fn is_complete_and_valid(&self) -> bool {
        todo!()
    }

    fn cell(&self, _x: usize, _y: usize) -> &Option<NonZero<u8>> {
        &None
    }

    fn step_count(&self) -> usize {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn backtracking_count(&self) -> usize {
        todo!()
    }

    fn invalid_attempts(&self) -> usize {
        todo!()
    }

    fn visited_cells_heatmap(&self) -> &[usize; 81] {
        todo!()
    }

    fn invalid_attempts_heatmap(&self) -> &[usize; 81] {
        todo!()
    }

    fn backtracked_heatmap(&self) -> &[usize; 81] {
        todo!()
    }

    fn solve(&mut self) -> Option<()> {
        todo!()
    }

    fn next(&mut self) -> Option<bool> {
        todo!()
    }
}
