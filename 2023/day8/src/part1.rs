use std::{collections::hash_map::HashMap, env, fs, process::exit};

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

type Directions = Vec<Direction>;

type NodeName = String;

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

#[derive(Debug)]
struct Node {
    name: NodeName,
    left: NodeName,
    right: NodeName,
}

type Nodes = HashMap<NodeName, Node>;

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

fn parse_input(input: &str) -> (Directions, Nodes) {
    let lines: Vec<&str> = input.lines().collect();

    let directions: Directions = lines[0]
        .chars()
        .map(|d| match d {
            'L' => Direction::LEFT,
            'R' => Direction::RIGHT,
            _ => exit(1),
        })
        .collect();

    let mut nodes: Nodes = HashMap::new();
    lines[2..].iter().for_each(|line| {
        let split: Vec<&str> = line.split(" = ").collect();

        let name: NodeName = split[0].into();
        let node: Vec<&str> = split[1]
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(", ")
            .collect();

        nodes.insert(
            name.clone(),
            Node {
                name: name.into(),
                left: node[0].into(),
                right: node[1].into(),
            },
        );
    });

    (directions, nodes)
}

fn follow_directions(directions: &Directions, nodes: &Nodes) -> usize {
    let mut num_steps = 0;

    let mut curr_node = nodes.get(START_NODE).unwrap();
    while curr_node.name != END_NODE {
        let direction = directions.get(num_steps % directions.len()).unwrap();
        let next_node = match direction {
            Direction::LEFT => &curr_node.left,
            Direction::RIGHT => &curr_node.right,
        };
        curr_node = nodes.get(next_node).unwrap();
        num_steps += 1;
    }

    num_steps
}

fn main() {
    let input = read_input();

    let (directions, nodes) = parse_input(&input);

    dbg!(&directions);
    dbg!(&nodes);

    let num_steps = follow_directions(&directions, &nodes);

    println!("Number of steps: {}", num_steps);
}
