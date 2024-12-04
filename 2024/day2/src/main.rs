use std::fs;
use std::io;

fn read_file_contents(file_path: &String) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_path.trim()).expect("Couldn't read file contents");
    let lines = contents.trim().split("\n");
    let mut rows = Vec::new();
    for line in lines {
        rows.push(
            line
                .split_whitespace()
                .map(|val| val.parse::<i32>().expect("Not a number"))
                .collect()
        )
    }
    rows
}

fn get_rows() -> Vec<Vec<i32>> {
    println!("Enter the file path:");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");
    read_file_contents(&file_path)
}

fn calculate_sign(left: i32, right: i32) -> i32 {
    if left < right {
        1
    }
    else {
        -1
    }
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    // First, find out whether increasing or decreasing
    let sign: i32 = calculate_sign(report[0], report[1]);
    for pair in report.windows(2) {
        let [i, j]: [i32; 2] = pair.try_into().unwrap();
        let diff = (j - i) * sign;
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn is_report_safe_dampened(report: &Vec<i32>, exclude: usize) -> bool {
    let sign: i32 = {
        if exclude == 0 {
            calculate_sign(report[1], report[2])
        }
        else if exclude == 1 {
            calculate_sign(report[0], report[2])
        }
        else {
            calculate_sign(report[0], report[1])
        }
    };
    for (i, window) in report.windows(3).enumerate() {
        let [a, b, c]: [i32; 3] = window.try_into().unwrap();
        let diff = {
            if exclude <= i {
                c - b
            }
            else if exclude == i+1 {
                c - a
            }
            else { // exclude >= i+2
                b - a
            }
        } * sign;
        if diff < 1 || diff > 3 {
            return false
        }
    }
    true
}

fn is_report_safe_with_dampener(report: &Vec<i32>) -> bool {
    // Iterate over all of the possible exclusions
    for exclude in 0..report.len() {
        if is_report_safe_dampened(report, exclude) {
            return true;
        }
    }
    false
}

fn count_safe_reports(reports: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for report in reports {
        if is_report_safe(&report) {
            total += 1;
        }
        else if is_report_safe_with_dampener(&report) {
            total += 1;
        }
    }
    total
}

fn main() {
    let reports = get_rows();
    println!("Safe reports: {}", count_safe_reports(&reports));
}
