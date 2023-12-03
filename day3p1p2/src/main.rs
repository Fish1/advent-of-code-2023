use std::fs;

fn main() {
    println!("Day 3 Puzzle 1 & Puzzle 2");

    let file = read_file("./input/day3p1.txt");
    let symbols = get_symbols(&file.data, file.width);
    let numbers = get_numbers(&file.data, file.width);
    let numbers_next_to_symbol = get_numbers_next_to_symbol(&numbers, &symbols);
    let sum = sum_numbers(numbers_next_to_symbol);
    let ratios = get_gear_ratios(&numbers, &symbols);
    let sum_ratios = sum_ratios(ratios);

    println!("sum = {}", sum);
    println!("sum_ratios = {}", sum_ratios);
}

struct File {
    data: String,
    width: usize,
}

fn read_file(filename: &str) -> File {
    let data = fs::read_to_string(filename).expect("failed to read file");
    // add one to account for the new line character
    let width = data.find('\n').expect("couldn't find new line") + 1;
    File { data, width }
}

#[derive(Debug)]
struct Number {
    value: u32,
    coords: Vec<(usize, usize)>,
}

#[derive(Debug)]
struct Symbol {
    value: char,
    coord: (usize, usize),
}

fn get_numbers(data: &String, width: usize) -> Vec<Number> {
    let mut result = vec![];
    let mut current: Option<Number> = None;
    for value in data.chars().enumerate() {
        let x = value.0 % width;
        let y = value.0 / width;
        let character = value.1;
        if character.is_numeric() {
            current = match current {
                Some(number) => Some(Number {
                    value: (number.value * 10) + character.to_digit(10).unwrap(),
                    coords: [number.coords, vec![(x, y)]].concat(),
                }),
                _ => Some(Number {
                    value: character.to_digit(10).unwrap(),
                    coords: vec![(x, y)],
                }),
            }
        } else {
            if let Some(number) = current {
                result.push(number);
                current = None;
            }
        }
    }
    return result;
}

fn get_symbols(data: &String, width: usize) -> Vec<Symbol> {
    data.chars()
        .enumerate()
        .filter(|c| c.1.is_numeric() == false && c.1 != '.' && c.1 != '\n')
        .map(|c| Symbol {
            value: c.1,
            coord: (c.0 % width, c.0 / width),
        })
        .collect()
}

fn get_numbers_next_to_symbol(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> Vec<u32> {
    let mut result = vec![];
    for number in numbers {
        'number_coord: for number_coord in &number.coords {
            let up = (number_coord.0, number_coord.1.saturating_sub(1));
            let right = (number_coord.0 + 1, number_coord.1);
            let down = (number_coord.0, number_coord.1 + 1);
            let left = (number_coord.0.saturating_sub(1), number_coord.1);
            let up_right = (number_coord.0 + 1, number_coord.1.saturating_sub(1));
            let down_right = (number_coord.0 + 1, number_coord.1 + 1);
            let down_left = (number_coord.0.saturating_sub(1), number_coord.1 + 1);
            let up_left = (
                number_coord.0.saturating_sub(1),
                number_coord.1.saturating_sub(1),
            );

            let checks = vec![
                up, right, down, left, up_right, down_right, down_left, up_left,
            ];

            for symbol in symbols {
                if checks.contains(&symbol.coord) {
                    result.push(number.value);
                    break 'number_coord;
                }
            }
        }
    }
    return result;
}

fn get_gear_ratios(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> Vec<(u32, u32)> {
    let mut result = vec![];

    let gears = symbols
        .iter()
        .filter(|s| s.value == '*')
        .collect::<Vec<&Symbol>>();

    for gear in gears {
        let up = (gear.coord.0, gear.coord.1.saturating_sub(1));
        let right = (gear.coord.0 + 1, gear.coord.1);
        let down = (gear.coord.0, gear.coord.1 + 1);
        let left = (gear.coord.0.saturating_sub(1), gear.coord.1);
        let up_right = (gear.coord.0 + 1, gear.coord.1.saturating_sub(1));
        let down_right = (gear.coord.0 + 1, gear.coord.1 + 1);
        let down_left = (gear.coord.0.saturating_sub(1), gear.coord.1 + 1);
        let up_left = (
            gear.coord.0.saturating_sub(1),
            gear.coord.1.saturating_sub(1),
        );
        let checks = vec![
            up, right, down, left, up_right, down_right, down_left, up_left,
        ];
        let mut ratio: (Option<u32>, Option<u32>, bool) = (None, None, false);
        for number in numbers {
            for number_coord in &number.coords {
                if checks.contains(&number_coord) {
                    ratio = match ratio {
                        (None, None, false) => (Some(number.value), None, false),
                        (Some(v1), None, false) => (Some(v1), Some(number.value), true),
                        (Some(v1), Some(v2), true) => (Some(v1), Some(v2), false),
                        _ => ratio,
                    };
                    break;
                }
            }

            if let (Some(_), Some(_), false) = ratio {
                break;
            }
        }

        if ratio.2 == true {
            result.push((ratio.0.unwrap(), ratio.1.unwrap()));
        }
    }

    return result;
}

fn sum_numbers(numbers: Vec<u32>) -> u32 {
    numbers.into_iter().reduce(|a, b| a + b).unwrap_or(0)
}

fn sum_ratios(ratios: Vec<(u32, u32)>) -> u32 {
    ratios
        .into_iter()
        .map(|r| r.0 * r.1)
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

#[test]
fn example() {
    let file = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string();

    let width = file.find('\n').expect("couldn't find new line") + 1;
    let data = file; //.replace('\n', "");

    let symbols = get_symbols(&data, width);
    println!("symbols: {:?}", symbols);

    let numbers = get_numbers(&data, width);
    println!("numbers: {:?}", numbers);

    let numbers_next_to_symbol = get_numbers_next_to_symbol(&numbers, &symbols);
    println!("numbers_next_to_symbols: {:?}", numbers_next_to_symbol);

    let sum = sum_numbers(numbers_next_to_symbol);
    println!("sum: {}", sum);

    let gear_ratios = get_gear_ratios(&numbers, &symbols);
    println!("gear_ratios: {:?}", gear_ratios);

    let sum_ratios = sum_ratios(gear_ratios);
    println!("sum_ratios: {}", sum_ratios);

    assert!(sum == 4361);
    assert!(sum_ratios == 467835);
}
