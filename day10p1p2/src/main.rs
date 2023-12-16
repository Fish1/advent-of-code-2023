use std::fs;

fn main() {
    let data = readfile("./input/day10p1.txt").replace("\n", "");
    let mut map = Map::new(&data, (140, 140));
    let steps = map.step_to_end();
    println!("steps = {}", steps);
}

fn readfile(filename: &str) -> String {
    let x = fs::read_to_string(filename).unwrap();
    return x;
}

struct Map<'a> {
    step: i64,
    // original_string: String,
    data: &'a [u8],
    size: (i64, i64),
    starting_coord: (i64, i64),
    previous_coords: ((i64, i64), (i64, i64)),
    current_coords: ((i64, i64), (i64, i64)),
}

impl Map<'_> {
    fn new(original_string: &String, size: (i64, i64)) -> Map {
        let starting_index = original_string.find(|e| e == 'S').unwrap() as i64;
        let starting_x = starting_index % size.0;
        let starting_y = starting_index / size.0;
        let starting_coord = (starting_x, starting_y);
        let previous_coords = (starting_coord, starting_coord);
        let current_coords = (starting_coord, starting_coord);
        let mut new_map = Map {
            step: 1,
            // original_string,
            data: original_string.as_bytes(),
            size,
            starting_coord,
            previous_coords,
            current_coords,
        };
        new_map.first_step();
        new_map
    }

    fn get_tile(&self, x: i64, y: i64) -> Option<u8> {
        if x >= self.size.0 as i64 || y >= self.size.1 as i64 || x < 0 || y < 0 {
            None
        } else {
            let index = self.size.0 * y + x;
            let item = *self.data.get(index as usize).unwrap();
            Some(item)
        }
    }

    fn first_step(&mut self) {
        let x = self.starting_coord.0 as i64;
        let y = self.starting_coord.1 as i64;
        let left = self.get_tile(x - 1, y);
        let right = self.get_tile(x + 1, y);
        let up = self.get_tile(x, y - 1);
        let down = self.get_tile(x, y + 1);

        let mut dirs = vec![];

        if let Some(left) = left {
            if [b'F', b'L', b'-'].contains(&left) {
                dirs.push((x - 1, y));
            }
        }
        if let Some(right) = right {
            if [b'J', b'7', b'-'].contains(&right) {
                dirs.push((x + 1, y));
            }
        }
        if let Some(up) = up {
            if [b'F', b'7', b'|'].contains(&up) {
                dirs.push((x, y - 1));
            }
        }
        if let Some(down) = down {
            if [b'J', b'L', b'|'].contains(&down) {
                dirs.push((x, y + 1));
            }
        }

        self.current_coords = (*dirs.get(0).unwrap(), *dirs.get(1).unwrap());
    }

    fn step(&mut self) {
        let saved_current = self.current_coords.clone();

        let x_0 = self.current_coords.0 .0;
        let y_0 = self.current_coords.0 .1;
        let left_coord_0 = (x_0 - 1, y_0);
        let right_coord_0 = (x_0 + 1, y_0);
        let up_coord_0 = (x_0, y_0 - 1);
        let down_coord_0 = (x_0, y_0 + 1);
        let left_0 = self.get_tile(left_coord_0.0, left_coord_0.1);
        let right_0 = self.get_tile(right_coord_0.0, right_coord_0.1);
        let up_0 = self.get_tile(up_coord_0.0, up_coord_0.1);
        let down_0 = self.get_tile(down_coord_0.0, down_coord_0.1);

        let mut new_coord_0 = (0, 0);

        if let Some(left) = left_0 {
            if [b'F', b'L', b'-'].contains(&left) && self.previous_coords.0 != left_coord_0 {
                new_coord_0 = left_coord_0;
            }
        }
        if let Some(right) = right_0 {
            if [b'J', b'7', b'-'].contains(&right) && self.previous_coords.0 != right_coord_0 {
                new_coord_0 = right_coord_0;
            }
        }
        if let Some(up) = up_0 {
            if [b'F', b'7', b'|'].contains(&up) && self.previous_coords.0 != up_coord_0 {
                new_coord_0 = up_coord_0;
            }
        }
        if let Some(down) = down_0 {
            if [b'J', b'L', b'|'].contains(&down) && self.previous_coords.0 != down_coord_0 {
                new_coord_0 = down_coord_0;
            }
        }

        self.current_coords.0 = new_coord_0;

        let x_1 = self.current_coords.1 .0;
        let y_1 = self.current_coords.1 .1;
        let left_coord_1 = (x_1.saturating_sub(1), y_1);
        let right_coord_1 = (x_1 + 1, y_1);
        let up_coord_1 = (x_1, y_1.saturating_sub(1));
        let down_coord_1 = (x_1, y_1 + 1);
        let left_1 = self.get_tile(left_coord_1.0, left_coord_1.1);
        let right_1 = self.get_tile(right_coord_1.0, right_coord_1.1);
        let up_1 = self.get_tile(up_coord_1.0, up_coord_1.1);
        let down_1 = self.get_tile(down_coord_1.0, down_coord_1.1);

        let mut new_coord_1 = (0, 0);
        let current_character = self
            .get_tile(self.current_coords.0 .0, self.current_coords.0 .1)
            .unwrap() as char;

        if let Some(left) = left_1 {
            // println!("left = {}", left);
            if [b'F', b'L', b'-'].contains(&left)
                && self.previous_coords.1 != left_coord_1
                && ['|'].contains(&current_character) == false
            {
                new_coord_1 = left_coord_1;
            }
        }
        if let Some(right) = right_1 {
            // println!("right = {}", right);
            if [b'J', b'7', b'-'].contains(&right)
                && self.previous_coords.1 != right_coord_1
                && ['|'].contains(&current_character) == false
            {
                new_coord_1 = right_coord_1;
            }
        }
        if let Some(up) = up_1 {
            // println!("up = {}", up as char);
            if [b'F', b'7', b'|'].contains(&up)
                && self.previous_coords.1 != up_coord_1
                && ['-'].contains(&current_character) == false
            {
                new_coord_1 = up_coord_1;
            }
        }
        if let Some(down) = down_1 {
            // println!("down = {}", down as char);
            if [b'J', b'L', b'|'].contains(&down)
                && self.previous_coords.1 != down_coord_1
                && ['-'].contains(&current_character) == false
            {
                new_coord_1 = down_coord_1;
            }
        }

        /*
        println!(
            "curr = {:?}",
            self.get_tile(self.current_coords.1 .0, self.current_coords.1 .1)
                .unwrap() as char
        );
        */

        self.current_coords.1 = new_coord_1;

        self.previous_coords = saved_current;
        self.step += 1;
    }

    fn step_to_end(&mut self) -> i64 {
        println!("{:?}", self.current_coords);
        while self.current_coords.0 != self.current_coords.1 {
            self.step();
            println!("{:?}", self.current_coords);
        }
        self.step
    }
}

#[test]
fn test1() {
    let data = ".....
.S-7.
.|.|.
.L-J.
....."
        .replace("\n", "");

    let mut map = Map::new(&data, (5, 5));
    let steps = map.step_to_end();
    println!("steps = {}", steps);
    assert!(steps == 4);
}

#[test]
fn test2() {
    let data = "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
        .replace("\n", "");

    let mut map = Map::new(&data, (5, 5));
    let steps = map.step_to_end();
    println!("steps = {}", steps);
    assert!(steps == 8);
}
