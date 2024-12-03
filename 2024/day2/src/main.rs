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

fn is_report_safe(report: &Vec<i32>) -> bool {
    // First, find out whether increasing or decreasing
    let sign: i32 = {
        if report[0] < report[1] {
            1            
        }
        else {
            -1
        }
    };
    for pair in report.windows(2) {
        let [i, j]: [i32; 2] = pair.try_into().unwrap();
        let diff = (j - i) * sign;
        if diff < 1 || diff > 3 {
            return false
        }
    }
    true
}

fn count_safe_reports(reports: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for report in reports {
        if is_report_safe(&report) {
            total += 1;
        }
    }
    total
}

fn main() {
    println!("Hello, world!");
    let reports = get_rows();
    println!("Safe reports: {}", count_safe_reports(&reports));
}
