use std::{env, fs, process::exit };

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    actual_numbers: Vec<usize>,
    numbers_intersection: Vec<usize>,
    score: usize,
}

type Cards = Vec<Card>;

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

fn parse_line(line: &str) -> Card {
    let split: Vec<&str> = line.split(":").collect();

    let id: usize = split
        .get(0)
        .unwrap()
        .strip_prefix("Card")
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();

    let numbers: Vec<&str> = split.get(1).unwrap().trim().split("|").collect();

    let winning_numbers: Vec<usize> = numbers
        .get(0)
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|num| num.len() > 0)
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let actual_numbers: Vec<usize> = numbers
        .get(1)
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|num| num.len() > 0)
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let numbers_intersection: Vec<usize> = winning_numbers
        .iter()
        .filter(|num| actual_numbers.contains(num))
        .map(|num| num.clone().to_owned())
        .collect();

    let score = match numbers_intersection.len() {
        0 => 0,
        n => 2usize.pow((n - 1).try_into().unwrap()),
    };

    Card {
        id,
        winning_numbers,
        actual_numbers,
        numbers_intersection,
        score,
    }
}

fn parse_input(input: &str) -> Cards {
    let mut cards = vec![];

    for line in input.lines() {
        cards.push(parse_line(&line));
    }

    cards
}

fn main() {
    let input = read_input();

    let cards = parse_input(&input);

    let score_sum: usize = cards.iter().fold(0, |acc, card| acc + card.score);

    // dbg!(cards);
    // dbg!(cards.iter().map(|card| card.score).collect::<Vec<usize>>());

    println!("Score sum: {}", score_sum);
}
