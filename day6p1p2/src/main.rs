use std::{cmp::Ordering, fs, sync::Arc};

fn main() {
    println!("Hello, world!");
    let text = fs::read_to_string("./input/day6p1.txt").unwrap();
    let race_list = Race::list(&text);
    let mut product = 1;
    for race in race_list {
        product *= race.count_winning_combinations();
    }
    println!("product = {}", product);

    let kerning = Race::kerning(&text);
    let kerning_wins = kerning.count_winning_combinations();
    println!("kerning wins = {}", kerning_wins);
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance_record: u64,
}

impl Race {
    pub fn list(text: &String) -> Vec<Race> {
        let lines = text.lines().collect::<Vec<&str>>();

        let times = lines
            .get(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect::<Vec<u64>>();

        let distances = lines
            .get(1)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect::<Vec<u64>>();

        times
            .iter()
            .enumerate()
            .map(|time| Race {
                time: *time.1,
                distance_record: *distances.get(time.0).unwrap(),
            })
            .collect()
    }

    pub fn kerning(text: &String) -> Race {
        let lines = text.lines().collect::<Vec<&str>>();

        let time = lines
            .get(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .replace(" ", "")
            .parse::<u64>()
            .unwrap();

        let distance_record = lines
            .get(1)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .replace(" ", "")
            .parse::<u64>()
            .unwrap();

        Race {
            time,
            distance_record,
        }
    }

    pub fn shortest_hold_to_win(&self) -> u64 {
        let data = (0..self.time).collect::<Arc<[u64]>>();
        let result = data.binary_search_by(|current| {
            let left_distance = hold_to_distance(current - 1, self.time);
            let current_distance = hold_to_distance(*current, self.time);
            if left_distance <= self.distance_record && current_distance > self.distance_record {
                return Ordering::Equal;
            }
            if left_distance > self.distance_record {
                return Ordering::Greater;
            }
            return Ordering::Less;
        });
        let result: u64 = result.unwrap().try_into().unwrap();
        return result + 1;
    }

    pub fn longest_hold_to_win(&self) -> u64 {
        let data = (0..self.time).collect::<Arc<[u64]>>();
        let result = data.binary_search_by(|current| {
            let right_distance = hold_to_distance(current + 1, self.time);
            let current_distance = hold_to_distance(*current, self.time);
            if right_distance <= self.distance_record && current_distance > self.distance_record {
                return Ordering::Equal;
            }
            if right_distance > self.distance_record {
                return Ordering::Less;
            }
            return Ordering::Greater;
        });
        let result: u64 = result.unwrap().try_into().unwrap();
        return result + 1;
    }

    pub fn count_winning_combinations(&self) -> u64 {
        let shortest = self.shortest_hold_to_win();
        let longest = self.longest_hold_to_win();
        (longest - shortest) + 1
    }
}

fn hold_to_distance(hold: u64, total: u64) -> u64 {
    assert!(total >= hold);
    (total - hold) * hold
}

#[test]
fn example() {
    let test = "Time: 7 15 30
Distance: 9 40 200"
        .to_string();

    let races = Race::list(&test);
    let mut product = 1;
    for race in races {
        product *= race.count_winning_combinations();
    }
    println!("product = {}", product);

    let kerning = Race::kerning(&test);
    let kerning_wins = kerning.count_winning_combinations();
    println!("kerning wins = {}", kerning_wins);

    assert!(product == 288);
    assert!(kerning_wins == 71503);
}
