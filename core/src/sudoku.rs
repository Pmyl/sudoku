use std::num::NonZero;

pub trait Sudoku {
    fn reset(&mut self);
    fn index(&self) -> Option<usize>;
    fn clues_count(&self) -> usize;
    fn is_complete_and_valid(&self) -> bool;
    fn cell(&self, x: usize, y: usize) -> &Option<NonZero<u8>>;
    fn step_count(&self) -> usize;
    fn is_valid(&self) -> bool;
    fn backtracking_count(&self) -> usize;
    fn invalid_attempts(&self) -> usize;
    fn visited_cells_heatmap(&self) -> &[usize; 81];
    fn invalid_attempts_heatmap(&self) -> &[usize; 81];
    fn backtracked_heatmap(&self) -> &[usize; 81];
    fn solve(&mut self) -> Option<()>;
    fn next(&mut self) -> Option<bool>;
}
