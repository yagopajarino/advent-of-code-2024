use std::env;
use std::fs;

fn parse_equations(lines: Vec<String>) -> Vec<(u128, Vec<u128>)> {
    lines
        .iter()
        .map(|line| {
            let mut parts = line.splitn(2, ":");
            let result: u128 = parts
                .next()
                .unwrap_or("")
                .trim()
                .parse()
                .expect("No se pudo parsear");
            let values: Vec<u128> = parts
                .next()
                .unwrap_or("")
                .trim()
                .split(" ")
                .filter(|item| *item != " ")
                .map(|f| {
                    f.parse::<u128>()
                        .expect(&format!("No se puede transformar {f} en integer"))
                })
                .collect();
            (result, values)
        })
        .collect()
}

fn integer_concat(a: u128, b: u128) -> u128 {
    let mut new_string: String = a.to_string();
    new_string.push_str(&b.to_string());
    new_string.parse::<u128>().unwrap()
}

fn is_valid_equation(partial: u128, expected: u128, i: usize, values: &Vec<u128>) -> bool {
    if i == values.len() {
        return partial == expected;
    }
    is_valid_equation(partial + values[i], expected, i + 1, values)
        || is_valid_equation(partial * values[i], expected, i + 1, values)
        || is_valid_equation(integer_concat(partial, values[i]), expected, i + 1, values)
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<String> = contents.lines().map(|line| String::from(line)).collect();
    let equations = parse_equations(lines);

    let mut res = 0;
    for equation in equations {
        if is_valid_equation(0, equation.0, 0, &equation.1) {
            res += equation.0;
        }
    }

    println!("{res}");
}
