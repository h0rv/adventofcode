use std::{env, fs, iter::zip, process::exit};

type Time = usize;
type Distance = usize;
type Times = Vec<Time>;
type Distances = Vec<Distance>;

type Speed = usize;

const STARTING_SPEED: usize = 0;

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

fn parse_input(input: &str) -> (Times, Distances) {
    let lines: Vec<&str> = input.lines().collect();

    let times: Times = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let distances: Distances = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    (times, distances)
}

fn get_speed(hold_for: Time) -> Speed {
    hold_for
}

fn number_of_ways_to_win(time: &Time, distance: &Distance) -> usize {
    let mut number_ways: usize = 0;

    (0..time + 1).for_each(|hold_for| {
        let speed = get_speed(hold_for);
        let moving_time = time - hold_for;
        let d = speed * moving_time;
        if d > *distance {
            dbg!(d);
            number_ways += 1;
        }
    });

    number_ways
}

fn number_of_ways_to_win_each(times: &Times, distances: &Distances) -> Vec<usize> {
    let mut number_ways: Vec<usize> = vec![];

    for (time, distance) in zip(times, distances) {
        number_ways.push(number_of_ways_to_win(time, distance));
    }

    number_ways
}

fn main() {
    let input = read_input();

    let (times, distances) = parse_input(&input);

    let number_ways = number_of_ways_to_win_each(&times, &distances);

    dbg!(&number_ways);

    println!(
        "Number of ways to win product: {}",
        number_ways.iter().product::<usize>()
    );
}
