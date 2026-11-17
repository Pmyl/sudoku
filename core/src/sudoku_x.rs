use std::num::NonZero;

use crate::sudoku::Sudoku;

pub struct SudokuX {}

impl SudokuX {
    pub fn new(_cells: [Option<NonZero<u8>>; 81]) -> SudokuX {
        SudokuX {}
    }
}

impl Sudoku for SudokuX {
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
