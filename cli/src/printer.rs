use core::sudoku::Sudoku;

pub fn print_board(sudoku: &Sudoku) {
    for row in sudoku.cells().array_chunks::<9>() {
        println!(
            "{}",
            row.iter()
                .map(|f| f.map(|v| v.to_string()).unwrap_or("□".to_string()))
                .collect::<String>()
        );
    }
}
