use std::fs;

fn main() {
    println!("Day 1 Puzzle 1");
    let data = read_file();
    let decoded_data = decode(data);
    let mut result = 0;
    for current in decoded_data {
        result += current;
    }
    println!("{result}");
}

fn decode(array: Vec<String>) -> Vec<u32> {
    let mut result = vec![];
    for line in array {
        let mut left_number: Option<u32> = None;
        let mut left_word = String::new();
        for character in line.chars() {
            match character.to_digit(10) {
                Some(number) => {
                    left_number = Some(number);
                    break;
                }
                _ => {
                    left_word.push(character);
                    let word_number = get_number_from_word(&left_word);
                    if let Some(number) = word_number {
                        left_number = Some(number);
                        break;
                    }
                }
            }
        }

        let mut right_number: Option<u32> = None;
        let mut right_word = String::new();
        for character in line.chars().rev() {
            match character.to_digit(10) {
                Some(number) => {
                    right_number = Some(number);
                    break;
                }
                _ => {
                    right_word.insert(0, character);
                    let word_number = get_number_from_word(&right_word);
                    if let Some(number) = word_number {
                        right_number = Some(number);
                        break;
                    }
                }
            }
        }

        let left = left_number.unwrap();
        let right = right_number.unwrap();
        result.push((left * 10) + right);
    }
    return result;
}

fn get_number_from_word(word: &String) -> Option<u32> {
    let numbers = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for number in numbers {
        if word.contains(number.0) {
            return Some(number.1);
        }
    }

    None
}

fn read_file() -> Vec<String> {
    fs::read_to_string("./input/day1p1.txt")
        .unwrap()
        .lines()
        .map(String::from)
        // ignore blank lines
        .filter(|x| x.len() > 0)
        .collect()
}

#[test]
fn example() {
    let test = vec![
        "two1nine".to_string(),
        "eightwothree".to_string(),
        "abcone2threexyz".to_string(),
        "xtwone3four".to_string(),
        "4nineeightseven2".to_string(),
        "zoneight234".to_string(),
        "7pqrstsixteen".to_string(),
        "2twoneh".to_string(),
    ];
    println!("{:?}", test);
    let result = decode(test);
    println!("{:?}", result);
    assert!(result[0] == 29);
    assert!(result[1] == 83);
    assert!(result[2] == 13);
    assert!(result[3] == 24);
    assert!(result[4] == 42);
    assert!(result[5] == 14);
    assert!(result[6] == 76);
    assert!(result[7] == 21);
}
