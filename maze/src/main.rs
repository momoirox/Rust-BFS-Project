use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Instant,
};

use state::{KeyState, State};
mod maze;
mod state;

pub fn costum_parse(input: String) -> VecDeque<String> {
    let split: VecDeque<String> = input.split(" ").map(|e: &str| e.to_string()).collect();

    return split;
}
fn main() {
    let path = Path::new("maze.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Can't open {}:{}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(&file);
    let mut x: VecDeque<String> = VecDeque::new();

    for line in reader.lines() {
        x.push_back(line.unwrap());
    }

    let height = 6;
    let width = 9;
    let mut maze = maze::Maze::new(width, height);

    for i in 0..height {
        for j in 0..width {
            let mut parsed_input = costum_parse(x.pop_front().unwrap());
            maze.add_cell(i, j, &mut parsed_input);
        }
    }
    //maze.print();
    let start = Instant::now();
    bfs_search(&mut maze);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

pub fn bfs_search(maze: &mut maze::Maze) {
    let mut frontier: VecDeque<State> = VecDeque::new();
    let mut visited: HashSet<State> = HashSet::new();
    let mut path: HashMap<State, State> = HashMap::new();

    let init_keys = VecDeque::new();
    let first_node = maze.cells[0].clone();

    let mut first_state = State::init(first_node, init_keys);
    let mut last_found: State = first_state.clone();
    //Ubaci prvo stanje
    frontier.push_front(first_state.clone());
    visited.insert(first_state.clone());

    while !frontier.is_empty() {
        let old_state = frontier.pop_front().unwrap();
        let mut cell = old_state.clone().cell;
        let old_keys = old_state.clone().keys;

        if cell.exit {
            println!(
                " END! x,y : ({}, {}) ",
                cell.coordinates.0, cell.coordinates.1
            );
            last_found = old_state;
            break;
        }

        for direction in cell.directions.iter() {
            let new_coordinates;
            let mut go = true;
            let mut new_keys = old_keys.clone();

            match direction {
                maze::Direction::East => {
                    new_coordinates = (cell.coordinates.0, cell.coordinates.1 + 1);
                }
                maze::Direction::West => {
                    new_coordinates = (cell.coordinates.0, cell.coordinates.1 - 1);
                }
                maze::Direction::North => {
                    new_coordinates = (cell.coordinates.0 - 1, cell.coordinates.1);
                }
                maze::Direction::South => {
                    new_coordinates = (cell.coordinates.0 + 1, cell.coordinates.1);
                }
            }
            for door in cell.doors.iter_mut() {
                match door {
                    maze::Door::East => {
                        if new_coordinates == (cell.coordinates.0, cell.coordinates.1 + 1) {
                            go = use_key(&mut new_keys);
                            *door = maze::Door::None;
                        }
                    }
                    maze::Door::West => {
                        if new_coordinates == (cell.coordinates.0, cell.coordinates.1 - 1) {
                            //ako ima kljuc moze ako nema ne moze
                            go = use_key(&mut new_keys);
                            *door = maze::Door::None;
                        }
                    }

                    maze::Door::North => {
                        if new_coordinates == (cell.coordinates.0 - 1, cell.coordinates.1) {
                            //ako ima kljuc moze ako nema ne moze
                            go = use_key(&mut new_keys);
                            *door = maze::Door::None;
                        }
                    }
                    maze::Door::South => {
                        if new_coordinates == (cell.coordinates.0 + 1, cell.coordinates.1) {
                            //ako ima kljuc moze ako nema ne moze
                            go = use_key(&mut new_keys);
                            *door = maze::Door::None;
                        }
                    }
                    maze::Door::None => {
                        go = true;
                    }
                }
            }

            if go == true {
                let new_cell = maze.clone().find_cell(new_coordinates).unwrap();

                if new_cell.key {
                    let key_taken = check_key_taken(&mut new_keys, new_coordinates);
                    if !key_taken {
                        new_keys.push_back(KeyState::new(new_coordinates, false).clone());

                        // println!(
                        //     "Child kupi novi kljuc na ({},{}) keys len ->{}",
                        //     new_coordinates.0,
                        //     new_coordinates.1,
                        //     new_keys.len()
                        // );
                    } else {
                        // println!("Key already taken");
                    }
                }

                let new_state = State::new(new_cell.clone(), new_keys.clone());

                if !visited.contains(&new_state) {
                    frontier.push_back(new_state.clone());
                    visited.insert(new_state.clone());
                    path.insert(new_state, old_state.clone());
                }
            }
        }
    }

    reconstruct_path(&mut last_found, path, &mut first_state);
}

pub fn reconstruct_path(
    last_found: &mut State,
    path: HashMap<State, State>,
    first_state: &mut State,
) -> () {
    let mut visited: VecDeque<State> = VecDeque::new();
    let mut child = last_found.clone();
    let mut parent = path.get(&child).unwrap();
    visited.push_front(child);
    visited.push_front(parent.clone());

    while parent != first_state {
        child = parent.clone();
        parent = path.get(&child).unwrap();
        visited.push_front(parent.clone());
    }

    for v in visited.iter() {
        println!(
            "(x,y) = ({},{}) ",
            v.cell.coordinates.0, v.cell.coordinates.1
        );
        //Ispis za kljuceve
        // for k in v.keys.iter() {
        //     println!(
        //         "Key found at (x,y) = ({},{}), and was used : {}",
        //         k.coordinates.0, k.coordinates.1, k.used
        //     );
        // }
    }
}
pub fn check_key_taken(keys: &mut VecDeque<KeyState>, new_key: (u32, u32)) -> bool {
    for k in keys {
        if k.coordinates == new_key {
            return true;
        }
    }

    return false;
}
pub fn use_key(keys: &mut VecDeque<KeyState>) -> bool {
    if keys.len() != 0 {
        for k in keys {
            if k.used == false {
                k.used = true;
                //println!("Using key {}", k.used);
                return true;
            }
        }
    }
    return false;
}
