use std::fs;
use std::io;

fn read_file_contents(file_path: &String) -> Vec<String> {
    let contents = fs::read_to_string(file_path.trim()).expect("Couldn't read file contents");
    let lines = contents
        .trim()
        .split("\n")
        .map(|line| line.to_string())
        .collect();
    lines
}

fn get_rows() -> Vec<String> {
    println!("Enter the file path:");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");
    read_file_contents(&file_path)
}

fn find_number(line: &str, start: usize) -> Option<(i32, usize)> {
    // Given a number and its starting position, find its end
    let mut end = start;
    while end < line.len() && line.chars().nth(end).unwrap().is_numeric() {
        end += 1;
    }
    if end > start {
        let val = line[start..end].parse::<i32>().unwrap();
        Some((val, end))
    } else {
        None
    }
}

fn parse_mul(line_segment: &str) -> Option<i32> {
    if &line_segment[..4] != "mul(" {
        return None;
    }
    // Find the first number
    let Some((first, first_end)) = find_number(line_segment, 4) else {
        return None;
    };
    if first_end >= line_segment.len() {
        return None;
    }
    match line_segment.chars().nth(first_end) {
        None => return None,
        Some(c) => {
            if c != ',' {
                return None;
            }
        }
    }
    let second_start = first_end + 1;
    let Some((second, second_end)) = find_number(line_segment, second_start) else {
        return None;
    };
    if second_end >= line_segment.len() {
        return None;
    }
    match line_segment.chars().nth(second_end) {
        None => return None,
        Some(c) => {
            if c != ')' {
                return None;
            }
        }
    }
    Some(first * second)
}

fn parse_do(line_segment: &str) -> bool {
    line_segment.len() >= 4 && &line_segment[0..4] == "do()"
}

fn parse_dont(line_segment: &str) -> bool {
    line_segment.len() >= 7 && &line_segment[0..7] == "don't()"
}

fn parse_line(line: &str, do_flag: &mut bool) -> i32 {
    let mut total = 0;
    for i in 0..(line.len() - 7) {
        if *do_flag {
            match parse_mul(&line[i..]) {
                Some(n) => {
                    total += n;
                    continue;
                },
                None => ()
            }
        }
        if parse_do(&line[i..]) {
            *do_flag = true;
        }
        else if parse_dont(&line[i..]) {
            *do_flag = false;
        }
    }
    total
}

fn parse_all_lines(lines: Vec<String>) -> i32 {
    let mut total = 0;
    let mut do_flag = true;
    for line in lines {
        total += parse_line(&line, &mut do_flag);
    }
    total
}

fn main() {
    let lines = get_rows();
    let val = parse_all_lines(lines);
    println!("{}", val);
}
