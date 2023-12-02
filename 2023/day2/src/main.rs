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
struct MinCubes {
    num_red: usize,
    num_green: usize,
    num_blue: usize,
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

#[derive(Debug)]
struct GameConfig {
    num_red: usize,
    num_green: usize,
    num_blue: usize,
}

#[derive(Debug)]
struct Games {
    games: Vec<Game>,
    config: GameConfig,
}

fn parse_cube(cube: &str) -> Cube {
    let mut split = cube.split_whitespace();
    Cube {
        number: split.next().unwrap().parse::<usize>().unwrap(),
        color: CubeColor::from_str(split.next().unwrap()).unwrap(),
    }
}

fn parse_input(input: &str, game_config: GameConfig) -> Games {
    let mut games: Vec<Game> = vec![];

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

    Games {
        games,
        config: game_config,
    }
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

impl Round {
    fn valid(&self, game_config: &GameConfig) -> bool {
        for cube in self.cubes.iter() {
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
}

impl Game {
    fn valid(&self, game_config: &GameConfig) -> bool {
        for round in self.rounds.iter() {
            if !round.valid(game_config) {
                return false;
            }
        }

        true
    }

    fn min(&self) -> MinCubes {
        let mut min_cubes = MinCubes {
            num_red: 0,
            num_green: 0,
            num_blue: 0,
        };

        for round in self.rounds.iter() {
            for cube in round.cubes.iter() {
                match cube.color {
                    CubeColor::Red => {
                        if cube.number > min_cubes.num_red {
                            min_cubes.num_red = cube.number;
                        }
                    }
                    CubeColor::Green => {
                        if cube.number > min_cubes.num_green {
                            min_cubes.num_green = cube.number;
                        }
                    }
                    CubeColor::Blue => {
                        if cube.number > min_cubes.num_blue {
                            min_cubes.num_blue = cube.number;
                        }
                    }
                }
            }
        }

        min_cubes
    }
}

impl Games {
    fn valid(&self) -> Vec<&Game> {
        let mut valid_games = vec![];

        for game in self.games.iter() {
            if game.valid(&self.config) {
                valid_games.push(game);
            }
        }

        valid_games
    }

    fn min_cubes(&self) -> Vec<MinCubes> {
        let mut min_cubes = vec![];

        for game in self.games.iter() {
            min_cubes.push(game.min());
        }

        min_cubes
    }
}

impl MinCubes {
    fn power(&self) -> usize {
        self.num_red * self.num_green * self.num_blue
    }
}

fn sum_of_mincubes_power(min_cubes: &Vec<MinCubes>) -> usize {
    let mut power = 0;
    for min_cube in min_cubes.iter() {
        power += min_cube.power();
    }
    power
}

fn main() {
    let (input, game_config) = read_input();
    let games = parse_input(&input, game_config);

    // let valid_games = games.valid();
    // let valid_games_ids_sum = valid_games.into_iter().fold(0, |acc, g| acc + g.id);
    // println!("\nValid games IDs sum: {}", valid_games_ids_sum)

    let min_cubes = games.min_cubes();
    // dbg!(min_cubes);

    let sum = sum_of_mincubes_power(&min_cubes);

    println!("\nPower of min cubes: {}", sum)
}
