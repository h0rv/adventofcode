use std::{env, fs, process::exit, str::FromStr};

#[derive(Debug)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl FromStr for CubeColor {
    type Err = ();
    fn from_str(input: &str) -> Result<CubeColor, ()> {
        match input {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Cube {
    number: usize,
    color: CubeColor,
}

#[derive(Debug)]
struct Round {
    cubes: Vec<Cube>,
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

type Games = Vec<Game>;

#[derive(Debug)]
struct GameConfig {
    num_red: usize,
    num_green: usize,
    num_blue: usize,
}

fn parse_cube(cube: &str) -> Cube {
    let mut split = cube.split_whitespace();
    Cube {
        number: split.next().unwrap().parse::<usize>().unwrap(),
        color: CubeColor::from_str(split.next().unwrap()).unwrap(),
    }
}

fn parse_input(input: &str) -> Games {
    let mut games: Games = vec![];

    for line in input.lines() {
        // println!("{}", line);

        let mut split = line.split(":");

        let id = split
            .next()
            .unwrap()
            .strip_prefix("Game ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut rounds: Vec<Round> = vec![];
        for cubes in split.next().unwrap().split(";") {
            let mut round = Round { cubes: vec![] };
            for cube in cubes.split(",") {
                round.cubes.push(parse_cube(cube));
            }
            rounds.push(round);
        }

        games.push(Game { id, rounds });
    }

    games
}

fn read_input() -> (String, GameConfig) {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Please provide an input file path");
        exit(1)
    }

    let input_path = args.get(1).unwrap();

    let open = fs::read_to_string(input_path);
    let input = match open {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Failed to open given file path. Error: {}", err);
            exit(1)
        }
    };

    (
        input,
        GameConfig {
            num_red: args.get(2).unwrap().parse().unwrap(),
            num_green: args.get(3).unwrap().parse().unwrap(),
            num_blue: args.get(4).unwrap().parse().unwrap(),
        },
    )
}

fn is_valid_round(round: &Round, game_config: &GameConfig) -> bool {
    for cube in round.cubes.iter() {
        match cube.color {
            CubeColor::Red => {
                if cube.number > game_config.num_red {
                    return false;
                }
            }
            CubeColor::Green => {
                if cube.number > game_config.num_green {
                    return false;
                }
            }
            CubeColor::Blue => {
                if cube.number > game_config.num_blue {
                    return false;
                }
            }
        }
    }

    true
}

fn is_valid_game(game: &Game, game_config: &GameConfig) -> bool {
    for round in game.rounds.iter() {
        if !is_valid_round(&round, game_config) {
            return false;
        }
    }

    true
}

fn get_valid_games(games: Games, game_config: GameConfig) -> Games {
    let mut valid_games = vec![];
    for game in games {
        if is_valid_game(&game, &game_config) {
            valid_games.push(game);
        }
    }
    valid_games
}

fn main() {
    let (input, game_config) = read_input();

    let games = parse_input(&input);
    let valid_games = get_valid_games(games, game_config);

    let valid_games_ids_sum = valid_games.into_iter().fold(0, |acc, g| acc + g.id);

    println!("\nValid games IDs sum: {}", valid_games_ids_sum)
}
