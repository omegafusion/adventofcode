use std::fs;
use std::io;
use itertools;


fn read_file_contents(file_path: &[str]) -> ([u32], [u32]) {
    let contents = fs::read_to_string(file_path)
    let lines = contents.trim().split("\n")
    let n = lines.len()
    let mut left: [i32] = [0;n]
    let mut right: [i32] = [0;n]
    for (i, &line) in lines.enumerate() {
        let line_split = line.split_whitespace()
        if line_split.len() > 2 {
            panic!("Too many numbers")
        }
        left[i] = line_split[0].parse().expect("Couldn't parse number");
        right[i] = line_split[1].parse().expect("Couldn't parse number");
    }
}

fn get_lists() -> ([u32], [u32]) {
    println!("Enter the file path:");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");

    let (left, right) = read_file_contents(&file_path);



    let first = get_number_list();
    println!("Enter the second list (space-separated)");
    let second = get_number_list();
    if first.len() != second.len() {
        panic!("Different length lists");
    }
    (first, second)
}

fn compute_difference(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let left_iter = itertools::sorted(left);
    let right_iter = itertools::sorted(right);
    let mut total = 0;
    for (i, j) in itertools::izip!(left_iter, right_iter) {
        total += (i - j).abs();
    }
    total
}

fn main() {
    let (list1, list2) = get_lists();
    let result = compute_difference(&list1, &list2);
    println!("{}", result)
}
