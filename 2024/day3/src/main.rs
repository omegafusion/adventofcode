fn find_number(line: &str, start: usize) -> Option<(i32, usize)> {
    // Given a number and its starting position, find its end
    let mut end = start;
    while end < line.len() && line.chars().nth(end).unwrap().is_numeric() {
        end += 1;
    }
    if end > start {
        let val = line[start..end].parse::<i32>().unwrap();
        Some((val, end))
    }
    else {
        None
    }
}

fn parse_line(line: &str) -> i32 {

    let mut total = 0;
    for i in 0..(line.len() - 7)  {
        if &line[i..i+4] != "mul(" {
            continue
        }
        let first_start = i+4;
        // Find the first number
        let Some((first, first_end)) = find_number(line, first_start) else {
            continue;
        };
        println!("{}", first_end);
        if first_end >= line.len() {
            continue
        }
        match line.chars().nth(first_end) {
            None => continue,
            Some(c) => {
                if c != ',' {
                    continue
                }
            }
        }
        let second_start = first_end + 1;
        let Some((second, second_end)) = find_number(line, second_start) else {
            continue;
        };
        if second_end >= line.len() {
            continue
        }
        match line.chars().nth(second_end) {
            None => continue,
            Some(c) => {
                if c != ')' {
                    continue
                }
            }
        }
        total += first * second
    }
    total
}

fn main() {
    println!("Hello, world!");
    let val = parse_line("mul(3,3)aammul(8,1)mul (1,3)_mulu(43,2)");
    println!("{}", val);
}
