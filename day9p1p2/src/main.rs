use std::fs;

fn main() {
    println!("Day 9 Puzzels 1 & 2");
    let text = read_file("./input/day9p1.txt");
    let next_sum = Series::list(&text).iter().map(|s| s.next()).sum::<i64>();
    println!("next sum = {}", next_sum);
    let prev_sum = Series::list(&text).iter().map(|s| s.prev()).sum::<i64>();
    println!("prev sum = {}", prev_sum);
}

#[derive(Debug)]
struct Series {
    data: Vec<i64>,
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

impl Series {
    fn new(text: &str) -> Series {
        Series {
            data: text
                .split_whitespace()
                .map(|t| t.parse().unwrap())
                .collect(),
        }
    }

    fn list(text: &str) -> Vec<Series> {
        text.lines().map(|line| Series::new(line)).collect()
    }

    pub fn next(&self) -> i64 {
        let ss = self.sub_series();
        ss.iter().map(|s| s.get(s.len() - 1).unwrap()).sum()
    }

    pub fn prev(&self) -> i64 {
        let ss = self.sub_series();
        ss.iter()
            .enumerate()
            .map(|s| s.1.get(0).unwrap() + ((s.0 as i64 % 2) * s.1.get(0).unwrap() * -2))
            .sum::<i64>()
    }

    fn sub_series(&self) -> Vec<Vec<i64>> {
        let mut result = vec![self.data.clone()];
        let mut prev_series = self.data.clone();
        let mut curr_series = vec![];
        while prev_series.iter().all(|e| *e == 0) == false {
            for p in prev_series.windows(2) {
                let prev = p.get(0).unwrap();
                let curr = p.get(1).unwrap();
                curr_series.push(curr - prev);
            }
            result.push(curr_series.clone());
            prev_series = curr_series;
            curr_series = vec![];
        }
        result
    }
}

#[test]
fn test() {
    let text = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    let next_sum = Series::list(text).iter().map(|s| s.next()).sum::<i64>();
    assert!(next_sum == 114);
    let prev_sum = Series::list(text).iter().map(|s| s.prev()).sum::<i64>();
    println!("prev_sum = {}", prev_sum);
    assert!(prev_sum == 2);
}
