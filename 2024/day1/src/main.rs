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

fn get_lists() -> (Vec<i32>, Vec<i32>) {
    println!("Enter the file path:");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");

    let (left, right) = read_file_contents(&file_path);



    // let first = get_number_list();
    // println!("Enter the second list (space-separated)");
    // let second = get_number_list();
    // if first.len() != second.len() {
    //     panic!("Different length lists");
    // }
    (left, right)
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

fn compute_similarity(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut left_sorted = left.clone();
    left_sorted.sort();
    let mut right_sorted = right.clone();
    right_sorted.sort();
    let mut total = 0;
    let mut l0 = 0;
    let mut l1 = 0;
    let mut r0 = 0;
    let mut r1 = 0;
    while l1 < left.len() && r1 < right.len() {
        if left_sorted[l0] == left_sorted[l1] {
            l1 += 1;
        }
        else if right_sorted[r0] == right_sorted[r1] {
            r1 += 1;
        }
        else {
            if left_sorted[l0] < right_sorted[r0] {
                l0 = l1;
            }
            else if left_sorted[l0] > right_sorted[r0] {
                r0 = r1;
            }
            else {
                let value = left_sorted[l0];
                let left_occurrences = l1 - l0;
                let right_occurrences = r1 - r0;
                total += value * (left_occurrences as i32 * right_occurrences as i32);
                l0 = l1;
                r0 = r1;
            }
        }
    }
    while l0 < left.len() && r1 < right.len() {
        // Left has reached the end, but right has not.
        // Treat the same as left reaching the next number.
        if right_sorted[r0] == right_sorted[r1] {
            r1 += 1;
        }
        else {
            if left_sorted[l0] < right_sorted[r0] {
                l0 = l1;
            }
            else if left_sorted[l0] > right_sorted[r0] {
                r0 = r1;
            }
            else {
                let value = left_sorted[l0];
                let left_occurrences = l1 - l0;
                let right_occurrences = r1 - r0;
                total += value * (left_occurrences as i32 * right_occurrences as i32);
                l0 = l1;
                r0 = r1;
            }
        }
    }
    while l1 < left.len() && r0 < right.len() {
        // Right has reached the end, but left has not.
        // Treat the same as right reaching the next number.
        if left_sorted[l0] == left_sorted[l1] {
            l1 += 1;
        }
        else {
            if left_sorted[l0] < right_sorted[r0] {
                l0 = l1;
            }
            else if left_sorted[l0] > right_sorted[r0] {
                r0 = r1;
            }
            else {
                let value = left_sorted[l0];
                let left_occurrences = l1 - l0;
                let right_occurrences = r1 - r0;
                total += value * (left_occurrences as i32 * right_occurrences as i32);
                l0 = l1;
                r0 = r1;
            }
        }
    }
    total
}

fn main() {
    let (list1, list2) = get_lists();
    let distance = compute_distance(&list1, &list2);
    println!("Distance: {}", distance);
    let similarity = compute_similarity(&list1, &list2);
    println!("Similarity: {}", similarity);
}
