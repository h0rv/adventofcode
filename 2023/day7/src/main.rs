use std::{cmp::Ordering, env, fs, iter::zip, process::exit};

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

type Card = usize;
type HandType = usize;
type Bid = usize;

type Hands = Vec<Hand>;
type Cards = Vec<Card>;
// type Cards = [Card; 5];

struct CardTypes;
#[derive(Debug)]
struct Hand {
    cards: Cards,
    bid: Bid,
    htype: HandType,
}

impl CardTypes {
    const ACE: Card = 13;
    const KING: Card = 12;
    const QUEEN: Card = 11;
    const TEN: Card = 10;
    const NINE: Card = 9;
    const EIGHT: Card = 8;
    const SEVEN: Card = 7;
    const SIX: Card = 6;
    const FIVE: Card = 5;
    const FOUR: Card = 4;
    const THREE: Card = 3;
    const TWO: Card = 2;
    const JOKER: Card = 1;

    fn get(card: char) -> Card {
        match card {
            'A' => Self::ACE,
            'K' => Self::KING,
            'Q' => Self::QUEEN,
            'J' => Self::JOKER,
            'T' => Self::TEN,
            '9' => Self::NINE,
            '8' => Self::EIGHT,
            '7' => Self::SEVEN,
            '6' => Self::SIX,
            '5' => Self::FIVE,
            '4' => Self::FOUR,
            '3' => Self::THREE,
            '2' => Self::TWO,
            _ => {
                eprintln!("Invalid input");
                exit(1);
            }
        }
    }
}

impl Hand {
    const FIVE_OF_A_KIND: HandType = 7;
    const FOUR_OF_A_KIND: HandType = 6;
    const FULL_HOUSE: HandType = 5;
    const THREE_OF_A_KIND: HandType = 4;
    const TWO_PAIR: HandType = 3;
    const ONE_PAIR: HandType = 2;
    const HIGH_CARD: HandType = 1;

    fn get_hand_type(cards: Cards) -> HandType {
        let mut counts: Vec<usize> = vec![];
        let mut num_jokers = 0;

        let mut tmp_cards: Vec<Card> = cards.clone().into();
        while tmp_cards.len() > 0 {
            let card = tmp_cards.get(0).unwrap().clone();
            let count = tmp_cards
                .iter()
                .fold(0, |cnt, crd| if *crd == card { cnt + 1 } else { cnt });

            if card == CardTypes::JOKER {
                num_jokers = count;
            } else {
                counts.push(count);
            }

            tmp_cards.retain(|crd| *crd != card);
        }

        counts.sort();

        dbg!(num_jokers);

        if num_jokers > 0 {
            match num_jokers {
                5 => Self::FIVE_OF_A_KIND,
                4 => Self::FIVE_OF_A_KIND,

                3 if counts.len() == 1 => Self::FIVE_OF_A_KIND,
                3 if counts.len() == 2 => Self::FOUR_OF_A_KIND,

                2 if counts.len() == 1 => Self::FIVE_OF_A_KIND,
                2 if counts.len() == 2 => Self::FOUR_OF_A_KIND,
                2 if counts.len() == 3 => Self::THREE_OF_A_KIND,

                1 if counts.len() == 1 => Self::FIVE_OF_A_KIND,
                1 if counts.len() == 2 && counts[0] == 1 => Self::FOUR_OF_A_KIND,
                1 if counts.len() == 2 && counts[0] == 2 => Self::FULL_HOUSE,
                1 if counts.len() == 3 => Self::THREE_OF_A_KIND,
                1 if counts.len() == 4 => Self::ONE_PAIR,

                _ => {
                    eprintln!("Joker error");
                    exit(1);
                }
            }
        } else {
            match counts.len() {
                1 => Self::FIVE_OF_A_KIND,
                2 if counts[1] == 4 => Self::FOUR_OF_A_KIND,
                2 if counts[1] == 3 => Self::FULL_HOUSE,
                3 if counts[2] == 3 => Self::THREE_OF_A_KIND,
                3 if counts[2] == 2 => Self::TWO_PAIR,
                4 if counts[3] == 2 => Self::ONE_PAIR,
                5 => Self::HIGH_CARD,
                _ => {
                    eprintln!("This shouldn't be able to happen");
                    exit(1);
                }
            }
        }
    }

    fn new(cards: Cards, bid: Bid) -> Self {
        Self {
            cards: cards.clone(),
            htype: Self::get_hand_type(cards),
            bid,
        }
    }
}

fn parse_input(input: &str) -> Hands {
    let mut hands: Hands = vec![];

    input.lines().for_each(|line| {
        let split: Vec<&str> = line.split_whitespace().collect();
        let (cards, bid): (Cards, Bid) = (
            split[0].chars().map(|card| CardTypes::get(card)).collect(),
            split[1].parse().unwrap(),
        );
        hands.push(Hand::new(cards, bid));
    });

    hands
}

fn main() {
    let input = read_input();

    let mut hands = parse_input(&input);

    hands.sort_by(|h1, h2| {
        if h1.htype < h2.htype {
            Ordering::Less
        } else if h1.htype > h2.htype {
            Ordering::Greater
        } else {
            // ==
            for (c1, c2) in zip(h1.cards.clone(), h2.cards.clone()) {
                if c1 < c2 {
                    return Ordering::Less;
                } else if c1 > c2 {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        }
    });

    // dbg!(&hands);

    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |sum, (i, hand)| sum + (i + 1) * hand.bid);

    println!("Total winnings: {}", total_winnings);
}
