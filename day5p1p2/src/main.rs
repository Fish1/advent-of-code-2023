use std::{cmp::Ordering, fs};

fn main() {
    println!("Hello, world!");
    let text = read_file("./input/day5p1.txt");
    let seeds = get_seeds(&text);
    let mut map_list = get_map_list(&text);
    let lowest_in_seeds = map_list.get_lowest_location_in_seeds(&seeds);
    println!("lowest: {}", lowest_in_seeds);
    let seed_ranges = get_seed_ranges(&seeds);
    let lowest_in_ranges = map_list.get_lowest_location_in_ranges(&seed_ranges);
    println!("lowest: {}", lowest_in_ranges);
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn get_seeds(text: &str) -> Vec<i64> {
    text.lines()
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn get_seed_ranges(seeds: &Vec<i64>) -> Vec<(i64, i64)> {
    seeds
        .iter()
        .enumerate()
        .step_by(2)
        .map(|s| (*s.1, seeds[s.0 + 1]))
        .collect()
}

fn get_map_list(text: &str) -> MapList {
    let mut results = vec![];
    let mut lines = text.lines();
    lines.nth(2);
    let mut current = Map::new();
    for line in lines {
        // if the line contains a : then it's another map
        if line == "" {
            current.offsets.sort_by_key(|m| m.1);
            results.push(current.clone());
            current = Map::new();
        } else if line.contains(":") {
        } else {
            let d = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>();
            let z = (d[0], d[1], d[2]);
            current.offsets.push(z);
        }
    }
    current.offsets.sort_by_key(|m| m.1);
    results.push(current.clone());
    MapList::from(results)
}

#[derive(Debug)]
struct MapList {
    pub maps: Vec<Map>,
}

impl MapList {
    pub fn from(data: Vec<Map>) -> MapList {
        MapList { maps: data }
    }

    pub fn get(&self, input: i64) -> i64 {
        let mut result = input;
        for map in &self.maps {
            result = map.get(result);
        }
        return result;
    }

    pub fn get_lowest_location_in_seeds(&self, seeds: &Vec<i64>) -> i64 {
        seeds.iter().map(|s| self.get(*s)).min().unwrap()
    }

    pub fn get_lowest_location_in_ranges(&self, seed_ranges: &Vec<(i64, i64)>) -> i64 {
        seed_ranges
            .iter()
            .map(|range| {
                let start = range.0;
                let end = range.0 + range.1;
                let mut min = i64::MAX;
                for c in start..end {
                    let location = self.get(c);
                    if location < min {
                        min = location;
                    }
                    if c % 10000000 == 0 {
                        println!("index: {}", c);
                    }
                }
                return min;
                /*
                (range.0..range.0 + range.1)
                    .map(|seed| {
                        if seed % 10000000 == 0 {
                            println!("index: {}", seed)
                        };
                        return self.get(seed);
                    })
                    .min()
                    .unwrap()
                    */
            })
            .min()
            .unwrap()
    }
}

#[derive(Debug, Clone)]
struct Map {
    pub offsets: Vec<(i64, i64, i64)>,
}

impl Map {
    pub fn new() -> Map {
        Map { offsets: vec![] }
    }

    pub fn get(&self, input: i64) -> i64 {
        let offset = self.offsets.binary_search_by(|probe| {
            if input < probe.1 {
                return Ordering::Greater;
            }
            if input >= probe.1 + probe.2 {
                return Ordering::Less;
            }
            return Ordering::Equal;
        });

        if let Ok(index) = offset {
            let offset = self.offsets.get(index).unwrap();
            let diff = offset.0 - offset.1;
            return input + diff;
        } else {
            return input;
        }
    }
}

#[test]
fn example() {
    let test = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    let seeds = get_seeds(test);
    let map_list = get_map_list(test);
    let lowest_in_seeds = map_list.get_lowest_location_in_seeds(&seeds);
    println!("lowest in seeds: {}", lowest_in_seeds);
    assert!(lowest_in_seeds == 35);
    let seed_ranges = get_seed_ranges(&seeds);
    let lowest_in_ranges = map_list.get_lowest_location_in_ranges(&seed_ranges);
    println!("lowest in range: {}", lowest_in_ranges);
    assert!(lowest_in_ranges == 46);
}
