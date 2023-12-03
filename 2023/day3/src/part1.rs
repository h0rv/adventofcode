use std::{env, fs, process::exit, usize};

type Schematic = Vec<Vec<char>>;

type IsValid = bool;
type PartNumber = usize;
type PartNumbers = Vec<PartNumber>;

fn read_input() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Please provide an input file path");
        exit(1)
    }

    let input_path = args.get(1).unwrap();

    match fs::read_to_string(input_path) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Failed to open given file path. Error: {}", err);
            exit(1)
        }
    }
}

fn parse_schematic(input: &str) -> Schematic {
    let mut schematic = vec![vec![]];

    input
        .lines()
        .for_each(|line| schematic.push(line.chars().collect()));

    schematic
}

fn print_schematic(schematic: &Schematic) {
    for row in schematic {
        for element in row {
            print!("{} ", element);
        }
        println!();
    }
}

fn is_symbol(ch: &char) -> bool {
    !ch.is_digit(10) && *ch != '.'
}

fn is_digit_valid(schematic: &Schematic, row_index: usize, col_index: usize) -> IsValid {
    let i_indices = if row_index == 0 {
        vec![row_index, row_index + 1]
    } else {
        vec![row_index - 1, row_index, row_index + 1]
    };
    let j_indices = if col_index == 0 {
        vec![col_index, col_index + 1]
    } else {
        vec![col_index - 1, col_index, col_index + 1]
    };

    for i in &i_indices {
        if let Some(row) = schematic.get(*i) {
            for j in &j_indices {
                if let Some(col) = row.get(*j) {
                    if is_symbol(col) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn get_part_number(
    schematic: &Schematic,
    row_index: usize,
    col_index: usize,
) -> (PartNumber, IsValid) {
    let mut part_number = 0;
    let mut is_valid = false;

    let row = &schematic[row_index];

    for (count, col) in row[col_index..].iter().enumerate() {
        if !col.is_digit(10) {
            break;
        }
        
        let j = col_index + count;

        part_number *= 10;
        part_number += col.to_digit(10).unwrap() as usize;

        // println!("Checking if {} is valid @({}, {})", col, row_index, j);
        is_valid |= is_digit_valid(schematic, row_index, j);
    }

    (part_number, is_valid)
}

fn get_part_numbers(schematic: &Schematic) -> PartNumbers {
    let mut part_numbers = vec![];

    let mut scan_til_next_non_digit = false;

    for (i, row) in schematic.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if !col.is_digit(10) {
                scan_til_next_non_digit = false;
                continue;
            };

            if scan_til_next_non_digit {
                continue;
            }

            if let (part_number, true) = get_part_number(&schematic, i, j) {
                part_numbers.push(part_number);
            }
            scan_til_next_non_digit = true;
        }
    }

    part_numbers
}

fn main() {
    let input = read_input();

    let schematic = parse_schematic(&input);

    print_schematic(&schematic);

    let part_numbers = get_part_numbers(&schematic);

    let part_number_sum: usize = part_numbers.iter().sum();

    println!("Sum of part numbers: {}", part_number_sum);
}
