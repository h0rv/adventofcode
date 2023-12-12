use std::{collections::HashMap, env, fs, process::exit};

#[derive(Eq, PartialEq, Clone, Debug)]
enum Area {
    EmptySpace,    // .
    Galaxy(usize), // #
}

type Row = Vec<Area>;
type Cord = (usize, usize);

#[derive(Clone, Debug)]
struct Image {
    image: Vec<Row>,
    galaxy_cords: HashMap<usize, Cord>,
}

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

impl Image {
    fn new_from_string(input: &str) -> Image {
        let mut image = Image {
            image: vec![],
            galaxy_cords: HashMap::new(),
        };

        let mut galaxy_num = 1;
        for line in input.lines() {
            let mut row: Row = vec![];
            for ch in line.chars() {
                match ch {
                    '.' => row.push(Area::EmptySpace),
                    '#' => {
                        row.push(Area::Galaxy(galaxy_num));
                        galaxy_num += 1;
                    }
                    _ => exit(1),
                }
            }
            image.image.push(row);
        }

        image.add_galaxy_cords();

        image
    }

    fn add_galaxy_cords(&mut self) {
        self.image.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, col)| match *col {
                Area::Galaxy(galaxy_num) => {
                    self.galaxy_cords.insert(galaxy_num, (i, j));
                }
                _ => (),
            })
        })
    }

    fn expand(&self) -> Image {
        let mut expanded = self.clone();
        expanded.galaxy_cords = HashMap::new();

        let mut rows_to_expand: Vec<usize> = vec![];
        for (i, row) in expanded.image.iter().enumerate() {
            let all_space = row.iter().all(|col| *col == Area::EmptySpace);

            if all_space {
                // expand
                let index = i + rows_to_expand.len(); // index will grow as earlier rows are added
                rows_to_expand.push(index);
            }
        }

        for index in rows_to_expand {
            let row = &expanded.image[index];

            expanded.image.insert(index, row.clone());
        }

        let mut cols_to_expand: Vec<usize> = vec![];
        for i in 0..expanded.image[0].len() {
            let all_space = self.image.iter().all(|row| row[i] == Area::EmptySpace);

            if all_space {
                // expand
                let index = i + cols_to_expand.len(); // index will grow as earlier rows are added
                cols_to_expand.push(index);
            }
        }

        for index in cols_to_expand {
            for row in expanded.image.iter_mut() {
                row.insert(index, Area::EmptySpace);
            }
        }

        expanded.add_galaxy_cords();

        expanded
    }

    fn get_galaxy_distances(&self) -> HashMap<(usize, usize), usize> {
        let mut distances = HashMap::new();

        let mut galaxy_nums: Vec<_> = self.galaxy_cords.keys().cloned().collect::<Vec<_>>();
        galaxy_nums.sort();

        // dbg!(&galaxy_nums);

        for (i, galaxy1) in galaxy_nums.iter().enumerate() {
            for galaxy2 in galaxy_nums[i + 1..].iter() {
                let cord1 = self.galaxy_cords[galaxy1];
                let cord2 = self.galaxy_cords[galaxy2];

                let (x1, y1) = cord1;
                let (x2, y2) = cord2;

                let distance = x1.abs_diff(x2) + y1.abs_diff(y2);

                distances.insert((*galaxy1, *galaxy2), distance);
            }
        }

        distances
    }

    fn print(&self) {
        for row in self.image.iter() {
            for col in row {
                match col {
                    Area::EmptySpace => print!("."),
                    Area::Galaxy(i) => print!("{i}"),
                }
            }
            println!();
        }
    }
}

fn main() {
    let input = read_input();

    let image = Image::new_from_string(&input);
    let expanded_image = image.expand();

    let distances = expanded_image.get_galaxy_distances();

    let distance_sum: usize = distances.values().sum();

    // dbg!(distances);

    println!("Distance sum: {distance_sum}");

    // image.print();
    // dbg!(image.galaxy_cords);
    //
    // println!();
    //
    // expanded_image.print();
    // dbg!(expanded_image.galaxy_cords);
}
