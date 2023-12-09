use std::{collections::HashMap, fs};

fn main() {
    println!("Hello, world!");
    let text = read_file("./input/day8p1.txt");
    let mut map = Map::new(&text);
    let steps = map.step_to_end();
    println!("steps = {}", steps);
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

struct Map {
    step: usize,
    location: String,
    instructions: String,
    routes: HashMap<String, (String, String)>,
}

impl Map {
    pub fn new(text: &str) -> Map {
        let lines = text.lines();
        let mut routes = HashMap::new();
        let mut instructions = String::new();
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
            }
        }
        Map {
            step: 0,
            location: String::from("AAA"),
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
}

#[test]
fn test() {
    let text = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    let mut map = Map::new(text);
    println!("map = {:?}", map.routes);
    let steps = map.step_to_end();
    println!("steps = {}", steps);
    assert!(steps == 6);
}
