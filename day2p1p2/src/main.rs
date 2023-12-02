use std::fs;

fn main() {
    println!("Day 2 Puzzle 1 & 2!");

    let data = read_file();
    let mut legal_sum = 0;
    let mut power_sum = 0;
    for line in data {
        let game = decode(line);
        power_sum += game.power();
        if game.is_legal() {
            legal_sum += game.id;
        }
    }
    println!("legal sum: {}", legal_sum);
    println!("power sum: {}", power_sum);
}

fn read_file() -> Vec<String> {
    fs::read_to_string("./input/day2p1.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

struct Game {
    id: u32,
    max: Set,
    sets: Vec<Set>,
}

impl Game {
    fn is_legal(&self) -> bool {
        for set in self.sets.clone() {
            if set.red > self.max.red || set.green > self.max.green || set.blue > self.max.blue {
                return false;
            }
        }
        true
    }

    fn power(&self) -> u32 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for set in self.sets.clone() {
            min_red = std::cmp::max(set.red, min_red);
            min_green = std::cmp::max(set.green, min_green);
            min_blue = std::cmp::max(set.blue, min_blue);
        }
        min_red * min_green * min_blue
    }
}

#[derive(Clone)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

fn decode(data: String) -> Game {
    let id = data
        .split(":")
        .nth(0)
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let data_sets = data.split(":").nth(1).unwrap().split(";");
    let mut sets = vec![];

    for data_set in data_sets {
        let colors = data_set.split(",");
        let mut new_set = Set {
            red: 0,
            green: 0,
            blue: 0,
        };
        for color in colors {
            let mut number_color = color.trim().split(" ");
            let number = number_color.next().unwrap().parse::<u32>().unwrap();
            let color = number_color.next().unwrap();
            if color == "red" {
                new_set.red = number;
            } else if color == "green" {
                new_set.green = number;
            } else if color == "blue" {
                new_set.blue = number;
            }
        }
        sets.push(new_set);
    }

    Game {
        id,
        sets,
        max: Set {
            red: 12,
            green: 13,
            blue: 14,
        },
    }
}

#[test]
fn example() {
    let tests = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
    ];

    let mut legals = vec![];
    let mut powers = vec![];
    for test in tests {
        let game = decode(test);
        legals.push(game.is_legal());
        powers.push(game.power());
    }

    assert!(legals[0] == true);
    assert!(legals[1] == true);
    assert!(legals[2] == false);
    assert!(legals[3] == false);
    assert!(legals[4] == true);

    assert!(powers[0] == 48);
    assert!(powers[1] == 12);
    assert!(powers[2] == 1560);
    assert!(powers[3] == 630);
    assert!(powers[4] == 36);
}
