use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))").unwrap();

    let mut res = 0;
    let mut multiplier = 1;
    for cap in re.captures_iter(&contents) {
        let matched = &cap[0];
        if matched.starts_with("do") {multiplier = 1;}
        if matched.starts_with("don't") {multiplier = 0;}
        if matched.starts_with("mul") {
            let parts: Vec<&str> = matched[4..matched.len()-1].split(',').collect();
            let left_op = parts[0].parse::<i32>().unwrap();
            let right_op = parts[1].parse::<i32>().unwrap();
            res += left_op * right_op * multiplier;
        }
    }
    println!("{}", res);   
}
