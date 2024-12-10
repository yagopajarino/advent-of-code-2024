use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut list_a: Vec<u32> = vec![];
    let mut list_b: Vec<u32> = vec![];
    for line in contents.lines() {
        let v: Vec<&str> = line.split(' ').filter(|x| *x != "").collect();
        list_a.push(v[0].parse().expect("No parseable"));
        list_b.push(v[1].parse().expect("No parseable"));
    }

    list_a.sort();
    list_b.sort();

    let star1: u32 = list_a
        .iter()
        .zip(list_b.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    println!("Star 1: {star1}");

    let mut recuentos_b: HashMap<u32, u32> = HashMap::new();
    for &element in &list_b {
        *recuentos_b.entry(element).or_insert(0) += 1;
    }

    let star2: u32 = list_a
        .iter()
        .map(|&a| a * recuentos_b.get(&a).copied().unwrap_or(0))
        .sum();

    println!("Star 2: {star2}");
}
