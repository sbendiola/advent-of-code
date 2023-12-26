use std::{collections::HashSet, fs, usize};

use regex::Regex;

pub fn main() {
    let total = fs::read_to_string("resources/input")
        .expect("failed opening file")
        .lines()
        .fold(0, |acc, line| {
            let card = card(line);
            let name = card.name.clone();
            let points = points(card);
            println!("{} {}", name, points);
            acc + points
        });

    println!("part {} {:?}", file!().to_string(), total);
}

struct Card {
    name: String,
    winners: HashSet<u8>,
    cards: Vec<u8>,
}

fn card(text: &str) -> Card {
    let re = Regex::new(r"^Card\s*(?P<name>\d+): (?P<winners>[^\|]+)\|(?P<cards>.*)$");

    let matched = re
        .expect("regex failed")
        .captures(text)
        .expect(format!("did not match {:?}", text).as_str());

    let name = matched.name("name").unwrap().as_str().trim();

    let winners: HashSet<u8> = matched
        .name("winners")
        .expect(&format!("failed on {:?}", matched.name("name")))
        .as_str()
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u8>().expect(&format!("failed to parse {}", s)))
        .collect();

    let cards = matched
        .name("cards")
        .unwrap()
        .as_str()
        .trim()
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u8>().expect(&format!("failed to parse {}", s)))
        .collect();

    Card {
        name: name.to_string(),
        winners: winners,
        cards: cards,
    }
}

fn points(card: Card) -> usize {
    let mut points = 0;
    let winners = card.winners;
    for card in card.cards {
        if winners.contains(&card) {
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }
    }
    points
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sample_input() {
        assert_eq!(
            points(card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")),
            8
        );
        assert_eq!(
            points(card("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")),
            2
        );
        assert_eq!(
            points(card("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")),
            2
        );
        assert_eq!(
            points(card("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83")),
            1
        );
        assert_eq!(
            points(card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")),
            0
        );
        assert_eq!(
            points(card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")),
            0
        );
    }
}
