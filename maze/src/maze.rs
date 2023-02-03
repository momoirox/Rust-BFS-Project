use std::collections::VecDeque;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Door {
    West,
    East,
    North,
    South,
    None,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    West,
    East,
    North,
    South,
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Cell {
    pub coordinates: (u32, u32),
    pub doors: VecDeque<Door>,
    pub directions: VecDeque<Direction>,
    pub key: bool,
    pub exit: bool,
}
impl Cell {
    pub fn new(
        x: u32,
        y: u32,
        doors: VecDeque<Door>,
        directions: VecDeque<Direction>,
        key: bool,
        exit: bool,
    ) -> Cell {
        Cell {
            coordinates: (x, y),
            doors: doors,
            directions: directions,
            key: key,
            exit: exit,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Maze {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

impl Maze {
    pub fn new(width: u32, height: u32) -> Maze {
        let cells = Vec::new();
        Maze {
            width,
            height,
            cells,
        }
    }
    pub fn print(self) -> () {
        print!(
            "Maze height: {}, Maze width: {} \n",
            self.height, self.width
        );
        for (_, el) in self.cells.iter().enumerate() {
            println!("Cell (x,y): ({}, {})", el.coordinates.0, el.coordinates.1);

            print!("Directions: ");
            for dir in el.directions.clone() {
                print!("{:?}, ", dir);
            }
            print!("Doors: ");
            for dir in el.doors.clone() {
                print!("{:?}, ", dir);
            }
            print!("Has key: {}", el.key);

            print!("EXIT: {}", el.exit);
            print!("\n");
        }
    }
    pub fn get_doors(input_door: String) -> VecDeque<Door> {
        let mut doors = VecDeque::new();
        for (i, el) in input_door.chars().enumerate() {
            if el == '1' {
                match i {
                    0 => doors.push_front(Door::West),
                    1 => doors.push_front(Door::East),
                    2 => doors.push_front(Door::North),
                    _ => doors.push_front(Door::South),
                };
            };
        }
        return doors;
    }

    pub fn get_directions(input_direction: String) -> VecDeque<Direction> {
        let mut directions = VecDeque::new();
        for (i, el) in input_direction.chars().enumerate() {
            if el == '1' {
                match i {
                    0 => directions.push_front(Direction::West),
                    1 => directions.push_front(Direction::East),
                    2 => directions.push_front(Direction::North),
                    _ => directions.push_front(Direction::South),
                };
            };
        }
        return directions;
    }
    pub fn add_cell(&mut self, x: u32, y: u32, input: &mut VecDeque<String>) -> &mut Maze {
        if input.len() != 3 {
            panic!("Input parse failed");
        } else {
            let input_direction = input.pop_front().unwrap();
            let input_door = input.pop_front().unwrap();
            let input_key_or_exit = input.pop_front().unwrap();

            let doors = Self::get_doors(input_door);
            let directions = Self::get_directions(input_direction);
            let mut exit = false;
            let mut key = false;
            match input_key_or_exit.as_str() {
                "0011" => exit = true,
                "1100" => key = true,
                _ => (),
            }

            let cell = Cell::new(x, y, doors, directions, key, exit);

            self.cells.push(cell);
            self
        }
    }

    pub fn find_cell(self, coordinates: (u32, u32)) -> Option<Cell> {
        for (_, el) in self.cells.iter().enumerate() {
            if el.coordinates.0 == coordinates.0 && el.coordinates.1 == coordinates.1 {
                return Some((*el).clone());
            }
        }

        return None;
    }

    pub fn key_found(&mut self, coordinates: (u32, u32)) -> () {
        for el in self.cells.iter_mut() {
            if el.coordinates.0 == coordinates.0 && el.coordinates.1 == coordinates.1 {
                el.key = false;
            }
        }
    }
}
