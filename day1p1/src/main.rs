use std::fs;

fn main() {
    println!("Day 1 Puzzle 1");
    let data = read_file();
    let result = decode(data);
    let mut sum = 0;
    for current in result.iter() {
        sum += current;
    }
    println!("{sum}");
}

fn decode(array: Vec<String>) -> Vec<u32> {
    let mut result = vec![];
    for line in array {
        let mut numbers: Option<[u32; 2]> = None;
        for character in line.chars() {
            match character.to_digit(10) {
                Some(value) => match numbers {
                    None => {
                        numbers = Some([value, value]);
                    }
                    Some(current) => numbers = Some([current[0], value]),
                },
                _ => {}
            }
        }
        let two_digit = match numbers {
            Some(values) => (values[0] * 10) + values[1],
            _ => 0,
        };
        result.push(two_digit);
    }
    return result;
}

fn read_file() -> Vec<String> {
    fs::read_to_string("./input/day1p1.txt")
        .unwrap()
        .lines()
        .map(|x| String::from(x))
        .filter(|x| x.len() > 0)
        .collect()
}

#[test]
fn example() {
    let test = vec![
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string(),
    ];
    println!("{:?}", test);

    let result = decode(test);
    println!("{:?}", result);
    assert!(result[0] == 12);
    assert!(result[1] == 38);
    assert!(result[2] == 15);
    assert!(result[3] == 77);
}
