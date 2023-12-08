use std::{cmp::Ordering, collections::HashMap, fs};

fn main() {
    println!("Day 7 Puzzels 1 & 2");

    let data = read_file("./input/day7p1.txt");
    let mut games = Game::parse(&data);
    games.sort_by(sort);
    let winnings = Game::count_winnings(&games);
    println!("winnings = {}", winnings);
    games.sort_by(sort_jack);
    let jack_winnings = Game::count_winnings(&games);
    println!("jack winnings = {}", jack_winnings);
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

#[derive(Debug)]
struct Game {
    hand: String,
    bid: i64,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum GameType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Game {
    fn new(hand: &str, bid: i64) -> Game {
        Game {
            hand: hand.to_owned(),
            bid,
        }
    }

    fn parse(data: &str) -> Vec<Game> {
        data.lines()
            .map(|line| {
                let mut data = line.split_whitespace();
                let hand = data.next().unwrap();
                let bid = data.next().unwrap().parse().unwrap();
                Game::new(hand, bid)
            })
            .collect()
    }

    fn count_winnings(games: &Vec<Game>) -> i64 {
        games
            .iter()
            .enumerate()
            .map(|game| game.1.bid * (game.0 + 1) as i64)
            .sum()
    }

    fn game_type(&self) -> GameType {
        let mut hand = HashMap::new();
        for character in self.hand.chars() {
            *hand.entry(character).or_insert(0) += 1;
        }
        if hand.len() == 5 {
            GameType::HighCard
        } else if hand.len() == 4 {
            GameType::OnePair
        } else if hand.len() == 3 {
            for card in hand {
                if card.1 == 3 {
                    return GameType::ThreeOfAKind;
                }
            }
            GameType::TwoPair
        } else if hand.len() == 2 {
            let first_card = *hand.iter().next().unwrap().1;
            if first_card == 4 || first_card == 1 {
                GameType::FourOfAKind
            } else {
                GameType::FullHouse
            }
        } else {
            GameType::FiveOfAKind
        }
    }

    fn game_type_jack(&self) -> GameType {
        let mut hand = HashMap::new();
        for character in self.hand.chars() {
            *hand.entry(character).or_insert(0) += 1;
        }
        let jack_count = *hand.get(&'J').unwrap_or(&0);
        if hand.len() == 5 {
            if jack_count == 1 {
                GameType::OnePair
            } else {
                GameType::HighCard
            }
        } else if hand.len() == 4 {
            if jack_count == 1 {
                GameType::ThreeOfAKind
            } else if jack_count == 2 {
                GameType::ThreeOfAKind
            } else {
                GameType::OnePair
            }
        } else if hand.len() == 3 {
            if jack_count == 2 || jack_count == 3 {
                return GameType::FourOfAKind;
            }
            for card in hand {
                if card.1 == 3 {
                    if jack_count == 1 {
                        return GameType::FourOfAKind;
                    } else {
                        return GameType::ThreeOfAKind;
                    }
                }
            }
            if jack_count == 1 {
                return GameType::FullHouse;
            } else {
                return GameType::TwoPair;
            }
        } else if hand.len() == 2 {
            if jack_count > 0 {
                return GameType::FiveOfAKind;
            }
            let first_card = *hand.iter().next().unwrap().1;
            if first_card == 4 || first_card == 1 {
                GameType::FourOfAKind
            } else {
                GameType::FullHouse
            }
        } else {
            GameType::FiveOfAKind
        }
    }
}

fn get_card_rank_jack(card: char) -> Result<i64, String> {
    match card {
        'J' => Ok(0),
        '2' => Ok(1),
        '3' => Ok(2),
        '4' => Ok(3),
        '5' => Ok(4),
        '6' => Ok(5),
        '7' => Ok(6),
        '8' => Ok(7),
        '9' => Ok(8),
        'T' => Ok(9),
        'Q' => Ok(11),
        'K' => Ok(12),
        'A' => Ok(13),
        _ => Err(String::from("not a valid card")),
    }
}

fn get_card_rank(card: char) -> Result<i64, String> {
    match card {
        '2' => Ok(1),
        '3' => Ok(2),
        '4' => Ok(3),
        '5' => Ok(4),
        '6' => Ok(5),
        '7' => Ok(6),
        '8' => Ok(7),
        '9' => Ok(8),
        'T' => Ok(9),
        'J' => Ok(10),
        'Q' => Ok(11),
        'K' => Ok(12),
        'A' => Ok(13),
        _ => Err(String::from("not a valid card")),
    }
}

fn sort(a: &Game, b: &Game) -> Ordering {
    let self_game_type = a.game_type();
    let other_game_type = b.game_type();
    if self_game_type < other_game_type {
        return Ordering::Less;
    } else if self_game_type > other_game_type {
        return Ordering::Greater;
    } else {
        let mut si = 0;
        let mut oi = 0;
        while si < a.hand.len() {
            let sc: char = *a.hand.chars().collect::<Vec<char>>().get(si).unwrap();
            let oc: char = *b.hand.chars().collect::<Vec<char>>().get(oi).unwrap();
            let sr = get_card_rank(sc);
            let or = get_card_rank(oc);
            if sr > or {
                return Ordering::Greater;
            } else if sr < or {
                return Ordering::Less;
            }
            si += 1;
            oi += 1;
        }
    }
    Ordering::Equal
}

fn sort_jack(a: &Game, b: &Game) -> Ordering {
    let self_game_type = a.game_type_jack();
    let other_game_type = b.game_type_jack();
    if self_game_type < other_game_type {
        return Ordering::Less;
    } else if self_game_type > other_game_type {
        return Ordering::Greater;
    } else {
        let mut si = 0;
        let mut oi = 0;
        while si < a.hand.len() {
            let sc: char = *a.hand.chars().collect::<Vec<char>>().get(si).unwrap();
            let oc: char = *b.hand.chars().collect::<Vec<char>>().get(oi).unwrap();
            let sr = get_card_rank_jack(sc);
            let or = get_card_rank_jack(oc);
            if sr > or {
                return Ordering::Greater;
            } else if sr < or {
                return Ordering::Less;
            }
            si += 1;
            oi += 1;
        }
    }
    Ordering::Equal
}

#[test]
fn test() {
    let data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
JJJ11 1
JJAA1 1
J1AA1 1";

    let mut games = Game::parse(data);
    games.sort_by(sort);
    let winnings = Game::count_winnings(&games);
    println!("winnings = {}", winnings);
    // assert!(winnings == 6440);
    games.sort_by(sort_jack);
    let jack_winnings = Game::count_winnings(&games);
    println!("jack winnings = {}", jack_winnings);
    for game in games {
        println!("game: {:?}, type: {:?}", game, game.game_type_jack());
    }
    // assert!(jack_winnings == 5905);
}
