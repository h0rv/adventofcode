use std::env;
use std::fs;
use std::process::exit;

const SPELLED_OUT_DIGITS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn spelled_out_digit(substr: &str, best_idx: &mut Option<usize>, reverse: bool) -> Option<u32> {
    let mut res = None;

    for (s, d) in SPELLED_OUT_DIGITS.iter() {
        let i = substr.find(s);
        if i.is_some() {
            let idx = i.unwrap();
            // println!("Found {} at {}", s, idx);
            if best_idx.is_none()
                || reverse && idx > best_idx.unwrap()
                || !reverse && idx < best_idx.unwrap()
            {
                res = Some(d.clone());
                *best_idx = Some(idx)
            }
        }
    }

    // println!(
    //     "substr: {} - best idx: {} - returning: {}",
    //     substr,
    //     best_idx.unwrap_or(usize::MAX),
    //     res.unwrap_or(u32::MAX)
    // );

    if res.is_none() {
        return res;
    }

    if !reverse {
        return res;
    }

    // Need to go recursive for rightmost find
    match get_first_digit(&substr[best_idx.unwrap() + 1..], reverse) {
        Some(best_res) => Some(best_res),
        None => res,
    }
}

fn get_first_digit(line: &str, reverse: bool) -> Option<u32> {
    let len = line.len();
    if reverse {
        for (i, c) in line.chars().rev().enumerate() {
            if c.is_numeric() {
                let idx = len - i - 1;
                // println!("idx: {} - char: {}", idx, c);
                // println!("{}", &line[i..]);
                match spelled_out_digit(&line[idx..], &mut None, reverse) {
                    Some(d) => return Some(d),
                    None => return c.to_digit(10),
                };
            }
        }
    } else {
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                let idx = i;
                match spelled_out_digit(&line[..idx], &mut Some(idx), reverse) {
                    Some(d) => return Some(d),
                    None => return c.to_digit(10),
                };
            }
        }
    }

    match spelled_out_digit(&line, &mut None, reverse) {
        Some(d) => return Some(d),
        None => {
            // eprintln!("No digit found in string");
            None
        }
    }
}

fn get_left_digit(line: &str) -> u32 {
    get_first_digit(line, false).unwrap()
}

fn get_right_digit(line: &str) -> u32 {
    get_first_digit(line, true).unwrap()
}

fn get_calibration_value_of_line(line: &str) -> u32 {
    let l = get_left_digit(line);
    let r = get_right_digit(line);
    let val = l * 10 + r;
    println!("{} - {}", line, val);
    val
}

fn get_calibration_value(input: String) -> u32 {
    input
        .lines()
        .fold(0, |acc, line| acc + get_calibration_value_of_line(line))
}

fn read_input() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Please provide an input file path");
        exit(1)
    }

    let input_path = args.get(1).unwrap();

    let open = fs::read_to_string(input_path);
    match open {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Failed to open given file path. Error: {}", err);
            exit(1)
        }
    }
}

fn main() {
    let input = read_input();

    let calibration_value = get_calibration_value(input);

    println!("\nCalibration value: {}", calibration_value);
}
