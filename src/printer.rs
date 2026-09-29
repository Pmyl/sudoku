use crate::Sudoku;

pub fn print_board(sudoku: &Sudoku) {
    for row in sudoku.cells.chunks_exact(9) {
        println!(
            "{}",
            row.iter()
                .map(|f| f.map(|v| v.to_string()).unwrap_or("□".to_string()))
                .collect::<String>()
        );
    }
}
