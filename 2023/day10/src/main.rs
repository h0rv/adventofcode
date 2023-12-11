use std::{collections::HashMap, env, fs, process::exit};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TileType {
    /*

    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

    */
    Vertical,
    Horizontal,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
    Covered(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum CameFrom {
    Above,
    Below,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tile {
    tile_type: TileType,
    pos: Pos,
    part_of_loop: bool,
}

impl Tile {
    fn new(tile_type: TileType, pos: Pos, part_of_loop: bool) -> Tile {
        Tile {
            tile_type,
            pos,
            part_of_loop,
        }
    }
}

type Tiles = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Diagram {
    tiles: Tiles,
    starting_position: Pos,
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

fn parse_input(input: &str) -> Diagram {
    let mut diagram = Diagram {
        tiles: vec![vec![]],
        starting_position: Pos { row: 0, col: 0 },
    };

    for (row, line) in input.lines().enumerate() {
        let mut tile_row = vec![];
        for (col, ch) in line.chars().enumerate() {
            tile_row.push(Tile {
                tile_type: match ch {
                    '|' => TileType::Vertical,
                    '-' => TileType::Horizontal,
                    'L' => TileType::NorthEastBend,
                    'J' => TileType::NorthWestBend,
                    '7' => TileType::SouthWestBend,
                    'F' => TileType::SouthEastBend,
                    '.' => TileType::Ground,
                    'S' => {
                        diagram.starting_position = Pos { row, col };
                        TileType::Start
                    }
                    _ => {
                        eprintln!("Invalid input: {}", ch);
                        exit(1);
                    }
                },
                pos: Pos { row, col },
                part_of_loop: false,
            })
        }
        diagram.tiles.push(tile_row);
    }

    // Hack to remove init []
    diagram.tiles.remove(0);

    diagram
}

impl Diagram {
    fn replace_start_tile(&mut self) {
        let (mut can_go_up, mut can_go_down, mut can_go_right, mut can_go_left): (
            bool,
            bool,
            bool,
            bool,
        ) = (false, false, false, false);

        let (row, col): (usize, usize) = (self.starting_position.row, self.starting_position.col);
        let (num_rows, num_cols): (usize, usize) = (self.tiles.len(), self.tiles[0].len());

        if row > 0 {
            can_go_up = match self.tiles[row - 1][col].tile_type {
                TileType::Vertical => true,
                TileType::SouthWestBend => true,
                TileType::SouthEastBend => true,
                _ => false,
            }
        }

        if row < num_rows - 1 {
            can_go_down = match self.tiles[row + 1][col].tile_type {
                TileType::Vertical => true,
                TileType::NorthWestBend => true,
                TileType::NorthEastBend => true,
                _ => false,
            }
        }

        if col > 0 {
            can_go_left = match self.tiles[row][col - 1].tile_type {
                TileType::Horizontal => true,
                TileType::NorthEastBend => true,
                TileType::SouthEastBend => true,
                _ => false,
            }
        }

        if col < num_cols - 1 {
            can_go_right = match self.tiles[row][col + 1].tile_type {
                TileType::Horizontal => true,
                TileType::NorthWestBend => true,
                TileType::SouthWestBend => true,
                _ => false,
            }
        }

        match (can_go_up, can_go_down, can_go_right, can_go_left) {
            (true, true, false, false) => self.tiles[row][col].tile_type = TileType::Vertical,
            (true, false, true, false) => self.tiles[row][col].tile_type = TileType::NorthEastBend,
            (true, false, false, true) => self.tiles[row][col].tile_type = TileType::NorthWestBend,
            (false, true, true, false) => self.tiles[row][col].tile_type = TileType::SouthEastBend,
            (false, true, false, true) => self.tiles[row][col].tile_type = TileType::SouthWestBend,
            (false, false, true, true) => self.tiles[row][col].tile_type = TileType::SouthWestBend,
            _ => {
                println!("({row}, {col})");
                dbg!(can_go_up);
                dbg!(can_go_down);
                dbg!(can_go_right);
                dbg!(can_go_left);
                exit(1);
            }
        }
    }

    fn traverser(&mut self, pos: Pos, came_from: CameFrom, start: bool) -> usize {
        let (row, col): (usize, usize) = (pos.row, pos.col);

        let mut curr_tile = &mut self.tiles[row][col];
        curr_tile.part_of_loop = true;

        if pos == self.starting_position && !start {
            // Base case - made it around!
            return 0;
        }

        // println!("row: {row}, col: {col})");
        // println!("came from:    {:?}", came_from);
        // println!("current tile: {:?}", curr_tile);

        match curr_tile.tile_type {
            TileType::Vertical => match came_from {
                CameFrom::Above => {
                    return 1 + self.traverser(Pos { row: row + 1, col }, CameFrom::Above, false)
                } // Go below
                CameFrom::Below => {
                    return 1 + self.traverser(Pos { row: row - 1, col }, CameFrom::Below, false)
                } // Go above
                _ => {
                    eprintln!("Invalid position: vertical");
                    exit(1);
                }
            },

            TileType::Horizontal => match came_from {
                CameFrom::Left => {
                    return 1 + self.traverser(Pos { row, col: col + 1 }, CameFrom::Left, false)
                } // Go right
                CameFrom::Right => {
                    return 1 + self.traverser(Pos { row, col: col - 1 }, CameFrom::Right, false)
                } // Go left
                _ => {
                    eprintln!("Invalid position: horizontal");
                    exit(1);
                }
            },

            TileType::NorthEastBend => match came_from {
                CameFrom::Above => {
                    return 1 + self.traverser(Pos { row, col: col + 1 }, CameFrom::Left, false)
                } // Go right
                CameFrom::Right => {
                    return 1 + self.traverser(Pos { row: row - 1, col }, CameFrom::Below, false)
                } // Go above
                _ => {
                    eprintln!("Invalid position: north east");
                    exit(1);
                }
            },

            TileType::NorthWestBend => match came_from {
                CameFrom::Above => {
                    return 1 + self.traverser(Pos { row, col: col - 1 }, CameFrom::Right, false)
                } // Go left
                CameFrom::Left => {
                    return 1 + self.traverser(Pos { row: row - 1, col }, CameFrom::Below, false)
                } // Go above
                _ => {
                    eprintln!("Invalid position: north west");
                    exit(1);
                }
            },

            TileType::SouthWestBend => match came_from {
                CameFrom::Below => {
                    return 1 + self.traverser(Pos { row, col: col - 1 }, CameFrom::Right, false)
                } // Go left
                CameFrom::Left => {
                    return 1 + self.traverser(Pos { row: row + 1, col }, CameFrom::Above, false)
                } // Go down
                _ => {
                    eprintln!("Invalid position: south west");
                    exit(1);
                }
            },

            TileType::SouthEastBend => match came_from {
                CameFrom::Below => {
                    return 1 + self.traverser(Pos { row, col: col + 1 }, CameFrom::Left, false)
                } // Go right
                CameFrom::Right => {
                    return 1 + self.traverser(Pos { row: row + 1, col }, CameFrom::Above, false)
                } // Go down
                _ => {
                    eprintln!("Invalid position: south east");
                    exit(1);
                }
            },

            _ => {
                eprint!("Invalid starting position: ");
                dbg!(curr_tile);
                exit(1);
            }
        }
    }

    fn traverse(&mut self) -> usize {
        // self.replace_start_tile();

        let (row, col): (usize, usize) = (self.starting_position.row, self.starting_position.col);

        let came_from = match self.tiles[row][col].tile_type {
            TileType::Vertical => CameFrom::Above,
            TileType::Horizontal => CameFrom::Left,
            TileType::NorthEastBend => CameFrom::Above,
            TileType::NorthWestBend => CameFrom::Above,
            TileType::SouthWestBend => CameFrom::Below,
            TileType::SouthEastBend => CameFrom::Below,
            _ => exit(1),
        };

        self.traverser(Pos { row, col }, came_from, true)
    }

    fn convert_not_in_loop_to_ground(&mut self) {
        self.tiles.iter_mut().for_each(|tile| {
            tile.iter_mut().for_each(|t| {
                if !t.part_of_loop {
                    t.tile_type = TileType::Ground
                }
            })
        })
    }

    fn find_first_gound(&self) -> Option<&Tile> {
        for tile in self.tiles.iter() {
            for t in tile.iter() {
                if t.tile_type == TileType::Ground {
                    return Some(t);
                }
            }
        }
        None
    }

    fn coverer(&mut self, pos: Pos, marker: usize) -> bool {
        let (row, col): (usize, usize) = (pos.row, pos.col);

        let (num_rows, num_cols): (usize, usize) = (self.tiles.len(), self.tiles[0].len());

        let mut curr_tile = &mut self.tiles[row][col];

        if curr_tile.tile_type != TileType::Ground {
            return false;
        }

        curr_tile.tile_type = TileType::Covered(marker);

        let mut seen_edge = false;
        // Up
        if row > 0 {
            seen_edge |= self.coverer(Pos { row: row - 1, col }, marker)
        } else {
            seen_edge = true;
        }

        // Down
        if row < num_rows - 1 {
            seen_edge |= self.coverer(Pos { row: row + 1, col }, marker)
        } else {
            seen_edge = true;
        }

        // Left
        if col > 0 {
            seen_edge |= self.coverer(Pos { row, col: col - 1 }, marker)
        } else {
            seen_edge = true;
        }

        // Right
        if col < num_cols - 1 {
            seen_edge |= self.coverer(Pos { row, col: col + 1 }, marker)
        } else {
            seen_edge = true;
        }

        // Up and to the left
        if row > 0 && col > 0 {
            seen_edge |= self.coverer(
                Pos {
                    row: row - 1,
                    col: col - 1,
                },
                marker,
            )
        }

        // Up and to the right
        if row > 0 && col < num_cols - 1 {
            seen_edge |= self.coverer(
                Pos {
                    row: row - 1,
                    col: col + 1,
                },
                marker,
            )
        }

        // Down and to the right
        if row < num_rows - 1 && col < num_cols - 1 {
            seen_edge |= self.coverer(
                Pos {
                    row: row + 1,
                    col: col + 1,
                },
                marker,
            )
        }

        // Down and to the left
        if row < num_rows - 1 && col > 0 {
            seen_edge |= self.coverer(
                Pos {
                    row: row + 1,
                    col: col - 1,
                },
                marker,
            )
        }

        return seen_edge;
    }

    fn cover_ground(&mut self) -> HashMap<usize, bool> {
        // Graph search to find groups of ground
        let mut mark_to_seen_edge: HashMap<usize, bool> = HashMap::new();
        for i in 1.. {
            let ground_tile: &Tile = match self.find_first_gound() {
                Some(tile) => tile,
                None => break,
            };

            let seen_edge = self.coverer(
                Pos {
                    row: ground_tile.pos.row,
                    col: ground_tile.pos.col,
                },
                i,
            );
            if seen_edge {
                mark_to_seen_edge.insert(i, seen_edge);
            }
        }

        mark_to_seen_edge
    }

    fn triple_scale(&self) -> Diagram {
        let (num_rows, num_cols): (usize, usize) = (self.tiles.len(), self.tiles[0].len());
        let (new_num_rows, new_num_cols): (usize, usize) = (num_rows * 3, num_cols * 3);

        println!("({num_rows}, {num_cols}) * 3 => ({new_num_rows}, {new_num_cols})");

        let mut new_diagram = Diagram {
            tiles: Vec::with_capacity(new_num_rows),
            starting_position: Pos {
                row: self.starting_position.row * 3 + 1,
                col: self.starting_position.col * 3 + 1,
            },
        };

        // Populate the new diagram with rows and columns with double capacity
        for _ in 0..new_num_rows {
            new_diagram.tiles.push(vec![
                Tile::new(
                    TileType::Ground,
                    Pos { row: 0, col: 0 },
                    false
                );
                new_num_cols
            ]);
        }

        for (row, tile) in self.tiles.iter().enumerate() {
            for (col, t) in tile.iter().enumerate() {
                let new_row = row * 3;
                let new_col = col * 3;
                // dbg!(new_row, new_col);

                // Set new block to ground
                new_diagram.tiles[new_row][new_col].pos = Pos {
                    row: new_row,
                    col: new_col,
                };
                new_diagram.tiles[new_row + 1][new_col].pos = Pos {
                    row: new_row + 1,
                    col: new_col,
                };
                new_diagram.tiles[new_row + 2][new_col].pos = Pos {
                    row: new_row + 2,
                    col: new_col,
                };
                new_diagram.tiles[new_row][new_col + 1].pos = Pos {
                    row: new_row,
                    col: new_col + 1,
                };
                new_diagram.tiles[new_row + 1][new_col + 1].pos = Pos {
                    row: new_row + 1,
                    col: new_col + 1,
                };
                new_diagram.tiles[new_row + 2][new_col + 1].pos = Pos {
                    row: new_row + 2,
                    col: new_col + 1,
                };
                new_diagram.tiles[new_row][new_col + 2].pos = Pos {
                    row: new_row,
                    col: new_col + 2,
                };
                new_diagram.tiles[new_row + 1][new_col + 2].pos = Pos {
                    row: new_row + 1,
                    col: new_col + 2,
                };
                new_diagram.tiles[new_row + 2][new_col + 2].pos = Pos {
                    row: new_row + 2,
                    col: new_col + 2,
                };

                match t.tile_type {
                    TileType::Vertical => {
                        /*
                         *        .|.
                         *   | -> .|.
                         *        .|.
                         */
                        new_diagram.tiles[new_row][new_col + 1] = Tile::new(
                            TileType::Vertical,
                            Pos {
                                row: new_row,
                                col: new_col + 1,
                            },
                            false,
                        );
                        new_diagram.tiles[new_row + 1][new_col + 1] = Tile::new(
                            TileType::Vertical,
                            Pos {
                                row: new_row + 1,
                                col: new_col + 1,
                            },
                            false,
                        );
                        new_diagram.tiles[new_row + 2][new_col + 1] = Tile::new(
                            TileType::Vertical,
                            Pos {
                                row: new_row + 2,
                                col: new_col + 1,
                            },
                            false,
                        );
                    }
                    TileType::Horizontal => {
                        new_diagram.tiles[new_row + 1][new_col] = Tile::new(
                            TileType::Horizontal,
                            Pos {
                                row: new_row + 1,
                                col: new_col,
                            },
                            false,
                        );
                        new_diagram.tiles[new_row + 1][new_col + 1] = Tile::new(
                            TileType::Horizontal,
                            Pos {
                                row: new_row + 1,
                                col: new_col + 1,
                            },
                            false,
                        );
                        new_diagram.tiles[new_row + 1][new_col + 2] = Tile::new(
                            TileType::Horizontal,
                            Pos {
                                row: new_row + 1,
                                col: new_col + 2,
                            },
                            false,
                        );
                    }
                    TileType::NorthEastBend => {
                        new_diagram.tiles[new_row][new_col + 1] = Tile::new(
                            TileType::Vertical,
                            Pos {
                                row: new_row,
                                col: new_col + 1,
                            },
                            false,
                        );
                        new_diagram.tiles[new_row + 1][new_col + 1] = Tile::new(
                            TileType::NorthEastBend,
                            Pos {
                                row: new_row + 1,
                                col: new_col + 1,
                            },
                            false,
                        );
                        new_diagram.tiles[new_row + 1][new_col + 2] = Tile::new(
                            TileType::Horizontal,
                            Pos {
                                row: new_row + 1,
                                col: new_col + 2,
                            },
                            false,
                        );
                    }
                    TileType::NorthWestBend => {
                        new_diagram.tiles[new_row][new_col + 1] = Tile::new(
                            TileType::Vertical,
                            Pos {
                                row: new_row,
                                col: new_col + 1,
                            },
                            false,
                        );
                        new_diagram.tiles[new_row + 1][new_col + 1] = Tile::new(
                            TileType::NorthWestBend,
                            Pos {
                                row: new_row + 1,
                                col: new_col,
                            },
                            false,
                        );
                        new_diagram.tiles[new_row + 1][new_col] = Tile::new(
                            TileType::Horizontal,
                            Pos {
                                row: new_row + 1,
                                col: new_col + 2,
                            },
                            false,
                        );
                    }
                    TileType::SouthWestBend => {
                        new_diagram.tiles[new_row + 1][new_col] = Tile::new(
                            TileType::Horizontal,
                            Pos {
                                row: new_row + 1,
                                col: new_col,
                            },
                            false,
                        );
                        new_diagram.tiles[new_row + 1][new_col + 1] = Tile::new(
                            TileType::SouthWestBend,
                            Pos {
                                row: new_row + 1,
                                col: new_col + 1,
                            },
                            false,
                        );
                        new_diagram.tiles[new_row + 2][new_col + 1] = Tile::new(
                            TileType::Vertical,
                            Pos {
                                row: new_row + 2,
                                col: new_col + 1,
                            },
                            false,
                        );
                    }
                    TileType::SouthEastBend => {
                        new_diagram.tiles[new_row + 1][new_col + 2] = Tile::new(
                            TileType::Horizontal,
                            Pos {
                                row: new_row + 1,
                                col: new_col + 2,
                            },
                            false,
                        );
                        new_diagram.tiles[new_row + 1][new_col + 1] = Tile::new(
                            TileType::SouthEastBend,
                            Pos {
                                row: new_row + 1,
                                col: new_col + 1,
                            },
                            false,
                        );
                        new_diagram.tiles[new_row + 2][new_col + 1] = Tile::new(
                            TileType::Vertical,
                            Pos {
                                row: new_row + 2,
                                col: new_col + 1,
                            },
                            false,
                        );
                    }
                    _ => (),
                }
            }
        }

        new_diagram
    }

    fn print(&self, mark_to_seen_edge: &HashMap<usize, bool>) {
        for tile in self.tiles.iter() {
            for t in tile.iter() {
                match t.tile_type {
                    TileType::Vertical => print!("|"),
                    TileType::Horizontal => print!("-"),
                    TileType::NorthEastBend => print!("L"),
                    TileType::NorthWestBend => print!("J"),
                    TileType::SouthWestBend => print!("7"),
                    TileType::SouthEastBend => print!("F"),
                    TileType::Ground => print!("."),
                    TileType::Start => print!("S"),
                    TileType::Covered(i) => {
                        if mark_to_seen_edge.get(&i).is_some() {
                            print!("O");
                        } else {
                            print!("I");
                        }
                    }
                }
            }
            println!();
        }
    }
}

fn main() {
    let input = read_input();

    let mut diagram = parse_input(&input);

    diagram.replace_start_tile();

    // dbg!(&diagram);

    let mut scaled_diagram = diagram.triple_scale();

    let distance = scaled_diagram.traverse();

    scaled_diagram.convert_not_in_loop_to_ground();

    let mark_to_seen_edge = scaled_diagram.cover_ground();

    let mut num_enclosed = 0;

    for (row, tile) in diagram.tiles.iter().enumerate() {
        for (col, _) in tile.iter().enumerate() {
            let new_row = row * 3 + 1;
            let new_col = col * 3 + 1;

            // dbg!(t.tile_type);

            // if t.tile_type != TileType::Ground {
            //     continue;
            // }

            match scaled_diagram.tiles[new_row][new_col].tile_type {
                TileType::Covered(i) => {
                    if !mark_to_seen_edge.get(&i).is_some() {
                        num_enclosed += 1;
                    }
                }
                _ => (),
            }
        }
    }

    diagram.print(&mark_to_seen_edge);
    scaled_diagram.print(&mark_to_seen_edge);

    let farthest_pos = distance / 2;

    println!("Distance traveled: {distance}");
    println!("Farthest position: {farthest_pos}");
    println!("Number enclosed:   {num_enclosed}");
}
