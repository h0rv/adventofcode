use num::integer::lcm;
use rayon::prelude::*;
use std::{collections::hash_map::HashMap, env, fs, process::exit};

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

type Directions = Vec<Direction>;

type NodeName = String;

#[derive(Debug)]
struct Node {
    name: NodeName,
    left: NodeName,
    right: NodeName,
    ends_with_z: bool,
    num_steps: usize,
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
                name: name.clone(),
                left: node[0].into(),
                right: node[1].into(),
                ends_with_z: name.ends_with("Z"),
                num_steps: 0,
            },
        );
    });

    (directions, nodes)
}

fn follow_directions(directions: &Directions, nodes: &Nodes) -> usize {
    let end_with_a: Vec<&String> = nodes.keys().filter(|node| node.ends_with("A")).collect();
    let end_with_z: Vec<&String> = nodes.keys().filter(|node| node.ends_with("Z")).collect();

    // let mut curr_nodes: Vec<&Node> = vec![nodes.get(*end_with_a.get(0).unwrap()).unwrap()];
    // let ends_with_z_nodes: Vec<&Node> = end_with_z.iter().map(|n| nodes.get(*n).unwrap()).collect();
    // assert_eq!(end_with_a.len(), end_with_z.len());
    // dbg!(&ends_with_z_nodes);

    let mut curr_nodes: Vec<&Node> = end_with_a.iter().map(|n| nodes.get(*n).unwrap()).collect();
    dbg!(&curr_nodes);

    // dbg!(direction);

    // Step all nodes
    let mut num_steps: Vec<usize> = curr_nodes
        .par_iter_mut()
        .map(|curr_node| {
            let mut num_steps = 0;
            loop {
                for direction in directions {
                    num_steps += 1;

                    // dbg!(&curr_node.name);
                    let next_node = match direction {
                        Direction::LEFT => &curr_node.left,
                        Direction::RIGHT => &curr_node.right,
                    };

                    *curr_node = &nodes[next_node];

                    if curr_node.ends_with_z {
                        dbg!(curr_node);
                        return num_steps;
                    }

                    // dbg!(&curr_nodes);
                    // dbg!(num_steps);
                }
            }
        })
        .collect();

    dbg!(&num_steps);

    if num_steps.len() == 1 {
        return num_steps[0];
    }

    let mut curr_lcm = num_steps.pop().unwrap().clone();
    while num_steps.len() > 0 {
        curr_lcm = lcm(curr_lcm, num_steps.pop().unwrap());
    }

    curr_lcm
}

fn main() {
    let input = read_input();

    let (directions, nodes) = parse_input(&input);

    let num_steps = follow_directions(&directions, &nodes);

    println!("Number of steps: {}", num_steps);
}
