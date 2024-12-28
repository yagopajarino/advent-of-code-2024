use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;
use std::io::{self};

fn parse_rules(lines: &[String]) -> Vec<(usize, usize)> {
    lines
        .iter()
        .filter(|line| !line.trim().is_empty() && !line.contains(','))
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            let x = parts[0].trim().parse::<usize>().unwrap();
            let y = parts[1].trim().parse::<usize>().unwrap();
            (x, y)
        })
        .collect()
}

fn parse_updates(lines: &[String]) -> Vec<Vec<usize>> {
    lines
        .iter()
        .filter(|line| line.contains(','))
        .map(|line| {
            line.split(',')
                .map(|num| num.trim().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn is_correct_order(update: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
    let page_position: HashMap<usize, usize> = update
        .iter()
        .enumerate()
        .map(|(idx, &page)| (page, idx))
        .collect();

    for &(x, y) in rules {
        if let (Some(&pos_x), Some(&pos_y)) = (page_position.get(&x), page_position.get(&y)) {
            if pos_x >= pos_y {
                return false;
            }
        }
    }

    true
}

fn topological_sort(update: &Vec<usize>, rules: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree: HashMap<usize, usize> = HashMap::new();

    for &page in update {
        in_degree.entry(page).or_insert(0);
    }

    for &(x, y) in rules {
        if update.contains(&x) && update.contains(&y) {
            adjacency.entry(x).or_insert(Vec::new()).push(y);
            *in_degree.entry(y).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    for (&page, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(page);
        }
    }

    let mut sorted: Vec<usize> = Vec::new();

    while let Some(page) = queue.pop_front() {
        sorted.push(page);

        if let Some(neighbors) = adjacency.get(&page) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    sorted
}

fn find_middle_page(sorted_update: &Vec<usize>) -> usize {
    let len = sorted_update.len();
    sorted_update[len / 2]
}

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let split_index = lines
        .iter()
        .position(|line| line.trim().is_empty())
        .unwrap_or(lines.len());
    let rules = parse_rules(&lines[..split_index]);
    let updates = parse_updates(&lines[split_index + 1..]);

    let mut sum_ordered_pages = 0;
    let mut sum_middle_pages: usize = 0;

    for update in updates {
        if !is_correct_order(&update, &rules) {
            let sorted_update = topological_sort(&update, &rules);
            let middle_page = find_middle_page(&sorted_update);
            sum_middle_pages += middle_page;
        } else {
            let middle_page = find_middle_page(&update);
            sum_ordered_pages += middle_page;
        }
    }

    println!("{}", sum_ordered_pages);
    println!("{}", sum_middle_pages);

    Ok(())
}
