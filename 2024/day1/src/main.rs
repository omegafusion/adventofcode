use std::fs;
use std::io;
use itertools;


fn read_file_contents(file_path: &String) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(file_path.trim()).expect("Couldn't read file contents");
    let lines = contents.trim().split("\n");
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in lines {
        let line_split = line.split_whitespace().collect::<Vec<_>>();
        if line_split.len() > 2 {
            panic!("Too many numbers");
        }
        left.push(line_split[0].parse().expect("Couldn't parse number"));
        right.push(line_split[1].parse().expect("Couldn't parse number"));
    }
    (left, right)
}

fn get_number_lists() -> (Vec<i32>, Vec<i32>) {
    println!("Enter the file path:");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");
    read_file_contents(&file_path)
}

fn compute_distance(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let left_sorted = itertools::sorted(left);
    let right_sorted = itertools::sorted(right);
    let mut total = 0;
    for (i, j) in itertools::izip!(left_sorted, right_sorted) {
        total += (i - j).abs();
    }
    total
}

fn compute_occurrences(vector: &Vec<i32>) -> Vec<(i32, usize)> {
    let mut sorted = vector.clone();
    sorted.sort();
    let mut paired: Vec<(i32, usize)> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while j < vector.len() {
        if sorted[i] == sorted[j] {
            j += 1;
        }
        else {
            let val = sorted[i];
            let count = j - i;
            paired.push((val, count));
            i = j
        }
    }
    if i < vector.len() {
        let val = sorted[i];
        let count = j - i;
        paired.push((val, count));
    }
    paired
}

fn compute_similarity(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut right_sorted = right.clone();
    right_sorted.sort();

    let left_paired = compute_occurrences(left);
    let right_paired = compute_occurrences(right);

    let mut total = 0;
    let mut l = 0;
    let mut r = 0;

    while l < left_paired.len() && r < right_paired.len() {
        let (left_val, left_occurrences) = left_paired[l];
        let (right_val, right_occurrences) = right_paired[r];
        if left_val < right_val {
            l += 1;
        }
        else if left_val > right_val {
            r += 1;
        }
        else {
            total += left_val * left_occurrences as i32 * right_occurrences as i32;
            l += 1;
            r += 1;
        }
    }
    total
}

fn main() {
    let (list1, list2) = get_number_lists();
    let distance = compute_distance(&list1, &list2);
    println!("Distance: {}", distance);
    let similarity = compute_similarity(&list1, &list2);
    println!("Similarity: {}", similarity);
}
