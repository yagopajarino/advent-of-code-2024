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
        line.split("").enumerate().for_each(|(col, value)| {
            matrix[fila][col] = value;
        });
        fila += 1;
    }
}
