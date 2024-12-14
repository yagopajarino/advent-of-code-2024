use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut matrix = vec![vec![""; lines.len()]; lines.len()];
    let mut fila = 0;
    for line in lines {
        line.split("")
            .filter(|x| *x != "")
            .enumerate()
            .for_each(|(col, value)| {
                matrix[fila][col] = value;
            });
        fila += 1;
    }

    // find xs
    let mut xs: Vec<(usize, usize)> = vec![];
    for (i, line) in matrix.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            if *cell == "X" {
                xs.push((i, j));
            }
        }
    }

    // for each x find a xmas
    let mut appearances = 0;
    for (fila, col) in xs {
        for direccion in 0..9 {
            if make_xmas("X", fila, col, direccion, &matrix) {
                appearances += 1;
            }
        }
    }

    fn next_letter_for(letter: &str) -> &str {
        if letter == "X" {
            return "M";
        };
        if letter == "M" {
            return "A";
        };
        "S"
    }

    fn get_cell_to_look(fila: usize, col: usize, direccion: i32, max_col: usize) -> (usize, usize) {
        if direccion == 0 && (fila > 0) && (col > 0) {
            return (fila - 1, col - 1);
        }
        if direccion == 1 && (fila > 0) {
            return (fila - 1, col);
        }
        if direccion == 2 && (fila > 0) && (col < max_col) {
            return (fila - 1, col + 1);
        }
        if direccion == 3 && (col > 0) {
            return (fila, col - 1);
        }
        if direccion == 5 && (col < max_col) {
            return (fila, col + 1);
        }
        if direccion == 6 && (fila < max_col) && (col > 0) {
            return (fila + 1, col - 1);
        }
        if direccion == 7 && (fila < max_col) {
            return (fila + 1, col);
        }
        if direccion == 8 && (fila < max_col) && col < max_col {
            return (fila + 1, col + 1);
        }
        (fila, col)
    }

    fn make_xmas(
        letter: &str,
        fila: usize,
        col: usize,
        direccion: i32,
        matrix: &Vec<Vec<&str>>,
    ) -> bool {
        if letter == "S" {
            return true;
        };
        let next_letter = next_letter_for(letter);
        let (fila_to_look, col_to_look) = get_cell_to_look(fila, col, direccion, matrix.len() - 1);
        if fila_to_look == fila && col_to_look == col {
            return false;
        }
        if matrix[fila_to_look][col_to_look] == next_letter {
            return make_xmas(next_letter, fila_to_look, col_to_look, direccion, matrix);
        }
        false
    }

    println!("Part 1: {appearances}");

    // Part 2
    let mut a_apps: Vec<(usize, usize)> = vec![];
    for (i, line) in matrix.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            if *cell == "A" {
                a_apps.push((i, j));
            }
        }
    }

    appearances = 0;
    for (fila, col) in a_apps {
        if does_make_x_mas(fila, col, &matrix) {
            appearances += 1;
        }
    }

    fn can_make_x(fila: usize, col: usize, matrix_size: usize) -> bool {
        fila > 0 && col > 0 && fila < matrix_size - 1 && col < matrix_size - 1
    }

    fn sort_string(input: &str) -> String {
        let mut chars: Vec<char> = input.chars().collect(); // Convert string to Vec<char>
        chars.sort(); // Sort the characters
        chars.into_iter().collect() // Convert back to String
    }

    fn does_make_x_mas(fila: usize, col: usize, matrix: &Vec<Vec<&str>>) -> bool {
        if !can_make_x(fila, col, matrix.len()) {
            return false;
        }
        let mut diag_uno = vec![
            matrix[fila - 1][col - 1],
            matrix[fila][col],
            matrix[fila + 1][col + 1],
        ]
        .join("");
        let mut diag_dos = vec![
            matrix[fila - 1][col + 1],
            matrix[fila][col],
            matrix[fila + 1][col - 1],
        ]
        .join("");
        diag_uno = sort_string(&diag_uno);
        diag_dos = sort_string(&diag_dos);
        (diag_dos == diag_uno) && diag_dos == "AMS"
    }

    println!("Part 2: {appearances}");
}
