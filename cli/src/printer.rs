use core::{sudoku::Sudoku, sudoku_array::SudokuArray};

pub fn print_board(sudoku: &SudokuArray) {
    for row in (0..81usize)
        .map(|i| sudoku.cell(i % 9, i / 9))
        .array_chunks::<9>()
    {
        println!(
            "{}",
            row.iter()
                .map(|f| f.map(|v| v.to_string()).unwrap_or("□".to_string()))
                .collect::<String>()
        );
    }
}
