use std::{env, fs, process::exit};

type Seed = usize;
type Seeds = Vec<Seed>;
type Ranges = Vec<RangeMap>;

#[derive(Debug)]
struct Range {
    min: usize,
    max: usize,
}

#[derive(Debug)]
struct RangeMap {
    source: Range,
    destination: Range,
}

impl Range {
    fn is_in(&self, number: usize) -> bool {
        self.min <= number && number <= self.max
    }
}

impl RangeMap {
    fn get(&self, number: usize) -> usize {
        // In source, then get it in destination; else return number
        if self.source.is_in(number) {
            let diff: usize = number - self.source.min;
            self.destination.min + diff
        } else {
            number
        }
    }
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

fn add_range(ranges: &mut Ranges, line: &str) {
    let split: Vec<&str> = line.split(" ").collect();

    let (destination_start, source_start, range_length): (usize, usize, usize) = (
        split.get(0).unwrap().parse().unwrap(),
        split.get(1).unwrap().parse().unwrap(),
        split.get(2).unwrap().parse().unwrap(),
    );

    // println!(
    //     "dst: {}, src: {}, range: {}",
    //     destination_start, source_start, range_length
    // );

    ranges.push(RangeMap {
        source: Range {
            min: source_start,
            max: source_start + range_length - 1,
        },
        destination: Range {
            min: destination_start,
            max: destination_start + range_length - 1,
        },
    });
    // for i in 0..range_length {
    // map.insert(source_start + i, destination_start + i);
    // map.insert(destination_start + i, source_start + i);
    // }
}

fn add_ranges(lines: &Vec<&str>, index: &mut usize, ranges: &mut Ranges) {
    loop {
        *index += 1;
        if let Some(range_line) = lines.get(*index) {
            if range_line.len() == 0 {
                return;
            }

            add_range(ranges, range_line);
        } else {
            return;
        }
    }
}

fn get(ranges: &Ranges, index: usize) -> usize {
    for range in ranges {
        if range.source.is_in(index) {
            return range.get(index);
        }
    }

    index
}

fn get_lowest(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    let mut index = 0;
    let seeds_split: Vec<&str> = lines
        .get(index)
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .collect();

    index += 1;

    let mut seed_to_soil = Ranges::new();
    let mut soil_to_fertilizer = Ranges::new();
    let mut fertilizer_to_water = Ranges::new();
    let mut water_to_light = Ranges::new();
    let mut light_to_temperature = Ranges::new();
    let mut temperature_to_humidity = Ranges::new();
    let mut humidity_to_location = Ranges::new();

    while index < lines.len() {
        let map_line = lines.get(index).unwrap();
        match map_line {
            line if line.starts_with("seed-to-soil map:") => {
                add_ranges(&lines, &mut index, &mut seed_to_soil)
            }
            line if line.starts_with("soil-to-fertilizer map:") => {
                add_ranges(&lines, &mut index, &mut soil_to_fertilizer)
            }
            line if line.starts_with("fertilizer-to-water map:") => {
                add_ranges(&lines, &mut index, &mut fertilizer_to_water)
            }
            line if line.starts_with("water-to-light map:") => {
                add_ranges(&lines, &mut index, &mut water_to_light)
            }
            line if line.starts_with("light-to-temperature map:") => {
                add_ranges(&lines, &mut index, &mut light_to_temperature)
            }
            line if line.starts_with("temperature-to-humidity map:") => {
                add_ranges(&lines, &mut index, &mut temperature_to_humidity)
            }
            line if line.starts_with("humidity-to-location map:") => {
                add_ranges(&lines, &mut index, &mut humidity_to_location)
            }
            _ => (),
        }
        index += 1;
    }

    println!("Maps populated!");

    // let mut locations: Vec<usize> = vec![];
    let mut lowest = usize::MAX;
    for i in (0..seeds_split.len()).step_by(2) {
        dbg!(i);
        let (start, length): (usize, usize) = (
            seeds_split.get(i).unwrap().parse().unwrap(),
            seeds_split.get(i + 1).unwrap().parse().unwrap(),
        );
        let end = start + length;
        (start..end).into_iter().for_each(|seed| {
            let location = get(
                &humidity_to_location,
                get(
                    &temperature_to_humidity,
                    get(
                        &light_to_temperature,
                        get(
                            &water_to_light,
                            get(
                                &fertilizer_to_water,
                                get(&soil_to_fertilizer, get(&seed_to_soil, seed)),
                            ),
                        ),
                    ),
                ),
            );
            if location < lowest {
                lowest = location;
            }
            // locations.push(location);

            // println!("Seed {}'s location: {}", seed, location);
        })
    }

    lowest
}

fn main() {
    let input = read_input();

    let lowest = get_lowest(&input);

    println!("Lowest location: {}", lowest);
}
