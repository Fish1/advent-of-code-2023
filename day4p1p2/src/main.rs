use std::{collections::HashMap, fs, sync::Arc};

fn main() {
    println!("Day 4 Puzzle 1 & Puzzel 2");
    let file_data = read_file("./input/day4p1.txt");
    let cards = lines_to_cards(file_data);
    let points = get_card_list_points(&cards);
    let card_count = get_card_count(&cards);
    println!("points = {}", points);
    println!("card_count = {}", card_count);
}

fn read_file(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[derive(Debug)]
struct Card {
    winning: Arc<[u32]>,
    have: Arc<[u32]>,
}

fn get_card_list_points(cards: &Vec<Card>) -> u32 {
    cards.iter().map(get_card_points).sum()
}

fn get_card_count(cards: &Vec<Card>) -> usize {
    let mut card_counts: HashMap<usize, usize> = HashMap::new();
    cards
        .iter()
        .enumerate()
        .map(|card| {
            let game_number = card.0;
            let wins = count_wins(card.1);
            let card_count = card_counts.get(&game_number).unwrap_or(&0).clone() + 1;
            let start_game = game_number + 1;
            let end_game = game_number + wins + 1;
            for game_number in start_game..end_game {
                *card_counts.entry(game_number).or_insert(0) += card_count;
            }
            return card_count;
        })
        .sum()
}

fn get_card_points(card: &Card) -> u32 {
    let mut points = 0;
    for have in card.have.iter() {
        if card.winning.contains(&have) {
            if points == 0 {
                points = 1;
            } else {
                points = points * 2;
            }
        }
    }
    return points;
}

fn count_wins(card: &Card) -> usize {
    card.have
        .iter()
        .filter(|c| card.winning.contains(c))
        .count()
}

fn lines_to_cards(lines: Vec<String>) -> Vec<Card> {
    lines
        .iter()
        .map(|line| {
            let data = line
                .split(':')
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .split('|')
                .collect::<Vec<&str>>();
            let winning: Arc<[u32]> = data
                .get(0)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|w| w.parse().unwrap())
                .collect();
            let have: Arc<[u32]> = data
                .get(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|h| h.parse().unwrap())
                .collect();
            Card { winning, have }
        })
        .collect()
}

#[test]
fn example() {
    let test = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let lines = test.lines().map(String::from).collect::<Vec<String>>();
    let cards = lines_to_cards(lines);
    let points = get_card_list_points(&cards);
    let cloned_count = get_card_count(&cards);
    println!("points = {}, clones = {}", points, cloned_count);
    assert!(points == 13);
    assert!(cloned_count == 30);
}
