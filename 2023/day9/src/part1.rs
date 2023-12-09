use std::{env, fs, process::exit};

type History = Vec<isize>;
type Histories = Vec<History>;

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

fn parse_input(input: &str) -> Histories {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.trim().parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}

fn get_diff(history: &History) -> History {
    let mut diff: History = vec![];

    (0..history.len()-1).for_each(|i| {
        let (l, r) = (history[i], history[i + 1]);
        diff.push(r - l);
    });

    diff
}

fn extrapolate(history: &History) -> isize {
    let mut diffs: Histories = vec![];

    let mut curr_diff = history.clone();
    loop {
        diffs.push((*curr_diff).to_vec());
        if curr_diff.iter().all(|value| *value == 0) {
            break;
        }
        curr_diff = get_diff(&curr_diff);
    }

    dbg!(&diffs);
    let extrapolation = diffs.iter_mut().map(|diff| diff.pop().unwrap()).sum();

    extrapolation
}

fn extrapolations(histories: &Histories) -> Vec<isize> {
    histories
        .iter()
        .map(|history| extrapolate(history))
        .collect()
}

fn main() {
    let input = read_input();

    let histories = parse_input(&input);

    let extrapolations = extrapolations(&histories);

    dbg!(&extrapolations);

    println!("Extrapolations: {}", extrapolations.iter().sum::<isize>());
}
