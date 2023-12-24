use std::fs;

fn main() {
    println!("Hello World!");

    let input = read_file("./input/day10p1.txt");
    let map = Map::new(&input);

    let starting_coord = map.get_starting_coord().unwrap();
    let starting_directions = map.get_valid_directions(starting_coord);

    let first = starting_directions.get(0).unwrap();
    let second = starting_directions.get(1).unwrap();
    let mut walker1 = Walker::new(&map, step(starting_coord, first), first.clone());
    let mut walker2 = Walker::new(&map, step(starting_coord, second), second.clone());
    let mut result = 1;
    while walker1.get_coord() != walker2.get_coord() {
        walker1.walk();
        walker2.walk();
        result += 1;
    }

    println!("result = {}", result);
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
    }
}

fn step(coord: (i64, i64), direction: &Direction) -> (i64, i64) {
    match direction {
        Direction::Left => (coord.0 - 1, coord.1),
        Direction::Right => (coord.0 + 1, coord.1),
        Direction::Up => (coord.0, coord.1 - 1),
        Direction::Down => (coord.0, coord.1 + 1),
    }
}

struct Walker<'a> {
    x: i64,
    y: i64,
    direction: Direction,
    map: &'a Map,
}

impl Walker<'_> {
    fn new(map: &Map, coord: (i64, i64), direction: Direction) -> Walker {
        Walker {
            x: coord.0,
            y: coord.1,
            direction: direction.clone(),
            map,
        }
    }

    fn get_coord(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    fn walk(&mut self) {
        let next_direction = self
            .map
            .get_next_direction((self.x, self.y), &opposite_direction(&self.direction))
            .unwrap();

        self.direction = next_direction;
        match self.direction {
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
            Direction::Up => {
                self.y -= 1;
            }
            Direction::Down => {
                self.y += 1;
            }
        }
    }
}

struct Map {
    data: String,
    width: usize,
    height: usize,
}

impl Map {
    fn new(text: &str) -> Map {
        let width = text.lines().next().unwrap().len();
        let height = text.lines().count();
        Map {
            data: text.to_string(),
            width,
            height,
        }
    }

    fn get_character(&self, coord: (i64, i64)) -> char {
        if coord.0 < 0
            || coord.0 >= self.width as i64
            || coord.1 < 0
            || coord.1 >= self.height as i64
        {
            return '.';
        }
        let x = coord.0 as usize;
        let y = coord.1 as usize;
        self.data.as_bytes()[(self.width + 1) * y + x] as char
    }

    fn get_starting_coord(&self) -> Option<(i64, i64)> {
        for x in 0..self.width {
            for y in 0..self.height {
                let coord = (x as i64, y as i64);
                let char = self.get_character(coord);
                if char == 'S' {
                    return Some(coord);
                }
            }
        }
        None
    }

    fn get_valid_directions(&self, coord: (i64, i64)) -> Vec<Direction> {
        let left = self.get_character((coord.0 - 1, coord.1));
        let right = self.get_character((coord.0 + 1, coord.1));
        let up = self.get_character((coord.0, coord.1 - 1));
        let down = self.get_character((coord.0, coord.1 + 1));
        let connects_left = ['-', 'L', 'F'].contains(&left);
        let connects_right = ['-', 'J', '7'].contains(&right);
        let connects_up = ['|', '7', 'F'].contains(&up);
        let connects_down = ['|', 'L', 'J'].contains(&down);

        let mut results = vec![];

        if connects_left {
            results.push(Direction::Left)
        }

        if connects_right {
            results.push(Direction::Right);
        }

        if connects_up {
            results.push(Direction::Up);
        }

        if connects_down {
            results.push(Direction::Down);
        }

        results
    }

    fn get_next_direction(&self, coord: (i64, i64), from: &Direction) -> Option<Direction> {
        let current = self.get_character(coord);
        let connects_left = ['-', 'J', '7'].contains(&current);
        let connects_right = ['-', 'L', 'F'].contains(&current);
        let connects_up = ['|', 'L', 'J'].contains(&current);
        let connects_down = ['|', '7', 'F'].contains(&current);

        if connects_left && from != &Direction::Left {
            Some(Direction::Left)
        } else if connects_right && from != &Direction::Right {
            Some(Direction::Right)
        } else if connects_up && from != &Direction::Up {
            Some(Direction::Up)
        } else if connects_down && from != &Direction::Down {
            Some(Direction::Down)
        } else {
            None
        }
    }
}

#[test]
fn test1() {
    let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    let map = Map::new(input);

    let starting_coord = map.get_starting_coord().unwrap();
    let starting_directions = map.get_valid_directions(starting_coord);

    let first = starting_directions.get(0).unwrap();
    let second = starting_directions.get(1).unwrap();
    let mut walker1 = Walker::new(&map, step(starting_coord, first), first.clone());
    let mut walker2 = Walker::new(&map, step(starting_coord, second), second.clone());
    let mut result = 1;
    while walker1.get_coord() != walker2.get_coord() {
        print!("w1 = {:?} -> ", walker1.get_coord(),);
        walker1.walk();
        println!("w1 = {:?}", walker1.get_coord(),);
        print!("w2 = {:?} -> ", walker2.get_coord(),);
        walker2.walk();
        println!("w2 = {:?}", walker2.get_coord(),);
        result += 1;
    }

    println!("result = {}", result);
    assert!(result == 4);
}

#[test]
fn test2() {
    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    let map = Map::new(input);

    let starting_coord = map.get_starting_coord().unwrap();
    let starting_directions = map.get_valid_directions(starting_coord);

    let first = starting_directions.get(0).unwrap();
    let second = starting_directions.get(1).unwrap();
    let mut walker1 = Walker::new(&map, step(starting_coord, first), first.clone());
    let mut walker2 = Walker::new(&map, step(starting_coord, second), second.clone());
    let mut result = 1;
    while walker1.get_coord() != walker2.get_coord() {
        print!("w1 = {:?} -> ", walker1.get_coord(),);
        walker1.walk();
        println!("w1 = {:?}", walker1.get_coord(),);
        print!("w2 = {:?} -> ", walker2.get_coord(),);
        walker2.walk();
        println!("w2 = {:?}", walker2.get_coord(),);
        result += 1;
    }

    println!("result = {}", result);
    assert!(result == 8);
}
