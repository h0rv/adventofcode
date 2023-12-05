use std::{env, fs, process::exit};

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    actual_numbers: Vec<usize>,
    number_winners: usize,
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

    let number_winners = numbers_intersection.len();

    Card {
        id,
        winning_numbers,
        actual_numbers,
        number_winners,
    }
}

fn parse_input(input: &str) -> Cards {
    let mut cards = vec![];

    for line in input.lines() {
        cards.push(parse_line(&line));
    }

    cards
}

fn calculate_instances(cards: &Cards) -> Vec<usize> {
    let mut instances = vec![1usize; cards.len()];

    for card in cards {
        let num_winners = card.number_winners;
        // println!(
        //     "\nCurrent card: {} - Number winners: {}\n",
        //     card.id, card.number_winners
        // );
        // dbg!(&instances);
        for add_to_card_id in card.id + 1..(card.id + num_winners + 1) {
            // dbg!(add_to_card_id);
            unsafe {
                let add_num: usize = *instances.get_unchecked_mut(card.id - 1);
                *instances.get_unchecked_mut(add_to_card_id - 1) += add_num;
            }
        }
    }

    instances
}

fn main() {
    let input = read_input();

    let cards = parse_input(&input);

    let instances = calculate_instances(&cards);

    let number_of_cards: usize = instances.iter().sum();

    // dbg!(cards);

    println!("Number of scratch cards: {}", number_of_cards);
}
