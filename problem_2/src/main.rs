use std::env;
use std::fs;

fn check_increasing(report: Vec<&str>) -> bool {
    let mut are_all_increasing = true;
    for i in 0..report.len() - 1 {
        let current = report[i].parse::<u32>().expect("Should have been able to parse");
        let next = report[i + 1].parse::<u32>().expect("Should have been able to parse");
        if current >= next {
            are_all_increasing = false;
            break;
        }
    }
    are_all_increasing
}

fn check_decreasing(report: Vec<&str>) -> bool {
    let mut are_all_decreasing = true;
    for i in 0..report.len() - 1 {
        let current = report[i].parse::<u32>().expect("Should have been able to parse");
        let next = report[i + 1].parse::<u32>().expect("Should have been able to parse");
        if current <= next {
            are_all_decreasing = false;
            break;
        }
    }
    are_all_decreasing
}

fn check_abs_diff_minor_than_4(report: Vec<&str>) -> bool {
    let mut are_all_abs_diff_minor_than_4 = true;
    for i in 0..report.len() - 1 {
        let current = report[i].parse::<u32>().expect("Should have been able to parse");
        let next = report[i + 1].parse::<u32>().expect("Should have been able to parse");
        if current.abs_diff(next) >= 4 {
            are_all_abs_diff_minor_than_4 = false;
            break;
        }
    }
    are_all_abs_diff_minor_than_4
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut safe_reports = 0;
    let mut unsafe_reports: Vec<Vec<&str>> = vec![];
    for line in contents.lines() {
        let report: Vec<&str> = line.split(' ').collect();
        let are_all_increasing = check_increasing(report.clone());
        let are_all_decreasing = check_decreasing(report.clone());
        let are_all_abs_diff_minor_than_4 = check_abs_diff_minor_than_4(report.clone());
        if (are_all_increasing || are_all_decreasing) && are_all_abs_diff_minor_than_4 {
            safe_reports += 1;
        } else {
            unsafe_reports.push(report);
        }
    }

    println!("Safe reports part 1: {}", safe_reports);

    // now for every unsafe check if removing one item makes it safe
    for unsafe_report in &unsafe_reports {
        for index in 0..unsafe_report.len() {
            let report_without_item: Vec<&str> = unsafe_report
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != index)
                .map(|(_, item)| *item)
                .collect();
            let are_all_increasing = check_increasing(report_without_item.clone());
            let are_all_decreasing = check_decreasing(report_without_item.clone());
            let are_all_abs_diff_minor_than_4 = check_abs_diff_minor_than_4(report_without_item.clone());
            if (are_all_increasing || are_all_decreasing) && are_all_abs_diff_minor_than_4 {
                safe_reports += 1;
                break;
            }
        }
    }
    println!("Safe reports part 2: {}", safe_reports);
}
