use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    println!("Day 8 Puzzels 1 & 2");
    let text = read_file("./input/day8p1.txt");
    let mut map = Map::new(&text);
    let steps = map.step_to_end();
    println!("steps = {}", steps);

    let mut map = Map::new(&text);
    let ghost_steps = map.ghost_step_to_end();
    println!("ghost steps = {}", ghost_steps);
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

struct Map {
    step: usize,
    location: String,
    ghost_location: HashSet<String>,
    instructions: String,
    routes: HashMap<String, (String, String)>,
}

impl Map {
    pub fn new(text: &str) -> Map {
        let lines = text.lines();
        let mut routes = HashMap::new();
        let mut instructions = String::new();
        let mut ghost_starting = HashSet::new();
        for line in lines.enumerate() {
            let line_number = line.0;
            if line_number == 0 {
                instructions = line.1.to_string();
            } else if line.1.len() == 0 {
                continue;
            } else {
                let with_removed_stuff = line
                    .1
                    .replace("=", "")
                    .replace("(", "")
                    .replace(",", "")
                    .replace(")", "");

                let mut split = with_removed_stuff.split_whitespace();
                let from = split.next().unwrap().to_string();
                let left = split.next().unwrap().to_string();
                let right = split.next().unwrap().to_string();
                routes.insert(from.clone(), (left, right));
                if from.as_bytes()[2] == b'A' {
                    ghost_starting.insert(from);
                }
            }
        }
        Map {
            step: 0,
            location: String::from("AAA"),
            ghost_location: ghost_starting,
            instructions: instructions.to_string(),
            routes,
        }
    }

    pub fn take_step(&mut self) {
        let direction = self.instructions.as_bytes()[self.step % self.instructions.len()];
        self.step += 1;
        let next_location = self.routes.get(&self.location).unwrap();
        if direction == b'L' {
            self.location = next_location.0.clone();
        } else {
            self.location = next_location.1.clone();
        }
    }

    pub fn step_to_end(&mut self) -> usize {
        while self.location != "ZZZ" {
            self.take_step();
        }
        self.step
    }

    pub fn step_to_first_z(&mut self) -> usize {
        while self.location.as_bytes()[2] != b'Z' {
            self.take_step();
        }
        self.step
    }

    pub fn ghost_step_to_end(&mut self) -> usize {
        let steps: Vec<usize> = self
            .ghost_location
            .clone()
            .iter()
            .map(|loc| {
                self.location = loc.clone();
                self.step = 0;
                self.step_to_first_z()
            })
            .collect();
        lcm(&steps)
    }
}

fn lcm(numbers: &Vec<usize>) -> usize {
    let gcd = gcd(numbers);
    numbers
        .iter()
        .skip(1)
        .fold(*numbers.get(0).unwrap(), |acc, e| (acc * e) / gcd)
}

fn gcd(numbers: &Vec<usize>) -> usize {
    let mut gcd = numbers.iter().min().unwrap().clone();
    while divises_all(gcd, &numbers) == false {
        gcd -= 1;
    }
    return gcd;
}

fn divises_all(divisor: usize, numbers: &Vec<usize>) -> bool {
    for n in numbers {
        if n % divisor != 0 {
            return false;
        }
    }
    true
}

#[test]
fn test() {
    let text = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    let mut map = Map::new(text);
    let steps = map.step_to_end();
    println!("steps = {}", steps);
    assert!(steps == 6);

    let text = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    let mut map = Map::new(text);
    let ghost_steps = map.ghost_step_to_end();
    println!("ghost steps = {}", ghost_steps);
    assert!(ghost_steps == 6);
}
