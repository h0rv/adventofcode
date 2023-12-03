use std::{env, fs, process::exit, usize};

type Schematic = Vec<Vec<char>>;

type Index = (usize, usize);
type Indices = Vec<Index>;
type IsValid = bool;
type GearRatio = usize;
type GearRatios = Vec<PartNumber>;
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

fn get_part_number(schematic: &Schematic, starting_index: Index) -> (PartNumber, IsValid) {
    let mut part_number = 0;
    let mut is_valid = false;

    let (row_index, col_index) = starting_index;

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

fn get_potential_gear_indices(schematic: &Schematic) -> Indices {
    let mut indices = vec![];

    for (i, row) in schematic.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '*' {
                indices.push((i, j));
            }
        }
    }

    indices
}

fn scan_num_right(schematic: &Schematic, starting_index: Index) -> PartNumber {
    let (part_number, _) = get_part_number(&schematic, starting_index);

    part_number
}

fn scan_num_left(schematic: &Schematic, index: Index) -> PartNumber {
    let (i, mut j) = index;

    let row = &schematic[i];

    for col in row[..j].iter().rev() {
        dbg!(col);
        if !col.is_digit(10) {
            break;
        }
        j -= 1;
    }

    scan_num_right(schematic, (i, j))
}

fn is_gear_index(schematic: &Schematic, index: &Index) -> IsValid {
    let mut num_above = 0;
    let mut num_below = 0;
    let mut num_left = 0;
    let mut num_right = 0;

    let (i, j) = *index;

    // Above
    if i > 0 {
        if schematic[i - 1][j].is_digit(10) {
            num_above = 1;
        } else {
            if j > 0 && schematic[i - 1][j - 1].is_digit(10) {
                num_above += 1
            }
            if j < schematic[i - 1].len() && schematic[i - 1][j + 1].is_digit(10) {
                num_above += 1
            }
        }
    }

    // Below
    if i < schematic.len() {
        if schematic[i + 1][j].is_digit(10) {
            num_below = 1;
        } else {
            if j > 0 && schematic[i + 1][j - 1].is_digit(10) {
                num_below += 1
            }
            if j <= schematic[i + 1].len() && schematic[i + 1][j + 1].is_digit(10) {
                num_below += 1
            }
        }
    }

    // Left
    if j > 0 {
        if schematic[i][j - 1].is_digit(10) {
            num_left = 1;
        }
    }

    // Right
    if j < schematic[i].len() {
        if schematic[i][j + 1].is_digit(10) {
            num_right = 1;
        }
    }

    num_above + num_below + num_left + num_right == 2
}

fn filter_potential_gear_indices(schematic: &Schematic, potential_indices: &Indices) -> Indices {
    let mut indices = vec![];

    for potential_index in potential_indices {
        if is_gear_index(schematic, potential_index) {
            indices.push(*potential_index);
        }
    }

    indices
}

fn get_gear_ratio(schematic: &Schematic, index: &Index) -> GearRatio {
    let mut gear_ratio = 1;

    let (i, j) = *index;

    // Above
    if i > 0 {
        if schematic[i - 1][j].is_digit(10) {
            dbg!(schematic[i - 1][j]);
            gear_ratio *= scan_num_left(schematic, (i - 1, j));
            dbg!(gear_ratio);
        } else {
            if j > 0 && schematic[i - 1][j - 1].is_digit(10) {
                dbg!(schematic[i - 1][j - 1]);
                gear_ratio *= scan_num_left(schematic, (i - 1, j - 1));
                dbg!(gear_ratio);
            }
            if j < schematic[i - 1].len() && schematic[i - 1][j + 1].is_digit(10) {
                gear_ratio *= scan_num_right(schematic, (i - 1, j + 1));
                dbg!(gear_ratio);
            }
        }
    }

    // Below
    if i < schematic.len() {
        if schematic[i + 1][j].is_digit(10) {
            gear_ratio *= scan_num_left(schematic, (i + 1, j));
            dbg!(gear_ratio);
        } else {
            if j > 0 && schematic[i + 1][j - 1].is_digit(10) {
                gear_ratio *= scan_num_left(schematic, (i + 1, j - 1));
                dbg!(gear_ratio);
            }
            if j <= schematic[i + 1].len() && schematic[i + 1][j + 1].is_digit(10) {
                gear_ratio *= scan_num_left(schematic, (i + 1, j + 1));
                dbg!(gear_ratio);
            }
        }
    }

    // Left
    if j > 0 {
        if schematic[i][j - 1].is_digit(10) {
            gear_ratio *= scan_num_left(schematic, (i, j - 1));
            dbg!(gear_ratio);
        }
    }

    // Right
    if j < schematic[i].len() {
        if schematic[i][j + 1].is_digit(10) {
            gear_ratio *= scan_num_right(schematic, (i, j + 1));
            dbg!(gear_ratio);
        }
    }

    gear_ratio
}

fn get_gear_ratios(schematic: &Schematic, gear_indicies: &Indices) -> GearRatios {
    let mut ratios = vec![];
    for index in gear_indicies.iter() {
        let (i, j) = *index;
        assert!(schematic[i][j] == '*');

        ratios.push(get_gear_ratio(schematic, index))
    }
    ratios
}

fn main() {
    let input = read_input();

    let schematic = parse_schematic(&input);

    print_schematic(&schematic);

    let potential_gear_indices = get_potential_gear_indices(&schematic);
    let gear_indicies = filter_potential_gear_indices(&schematic, &potential_gear_indices);
    let gear_ratios = get_gear_ratios(&schematic, &gear_indicies);
    let gear_ratios_sum: usize = gear_ratios.iter().sum();

    // dbg!(potential_gear_indices);
    // dbg!(gear_indicies);
    dbg!(gear_ratios);

    println!("Sum of gear ratios: {}", gear_ratios_sum);
}
