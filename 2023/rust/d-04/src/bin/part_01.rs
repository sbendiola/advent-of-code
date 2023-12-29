use std::{collections::HashSet, fs};

use regex::Regex;

pub fn main() {
    let total = fs::read_to_string("resources/input")
        .expect("failed opening file")
        .lines()
        .fold(0, |acc, line| {
            let card = card(line);
            let name = card.name.clone();
            let points = card.points();
            println!("{} {}", name, points);
            acc + points
        });

    println!("part {} {:?}", file!().to_string(), total);
}

struct Card {
    name: String,
    winners: HashSet<u8>,
    cards: HashSet<u8>,
}

impl Card {
    fn points(self) -> u32 {
        let winning_cards = self.cards.intersection(&self.winners).count();
        if winning_cards > 0 {
            1 << (winning_cards - 1)
        } else {
            0
        }
    }
}

fn card(text: &str) -> Card {
    let parts: Vec<&str> = text.split(":").collect();
    assert_eq!(parts.len(), 2);

    let re = Regex::new(r"^Card\s*(?P<name>\d+)");
    let matched = re
        .expect("regex failed")
        .captures(parts[0])
        .expect(format!("did not match {:?}", text).as_str());

    let name = matched.name("name").unwrap().as_str().trim();
    let parts: Vec<&str> = parts[1].split("|").collect();
    assert!(parts.len() == 2, "failed on {:?}", parts);

    let cards_result: Vec<HashSet<u8>> = parts
        .iter()
        .map(|text| {
            let result = text.split_whitespace().map(|s| s.parse::<u8>().unwrap());
            result.collect::<HashSet<u8>>()
        })
        .collect();

    if let [winners, cards] = cards_result.as_slice() {
        Card {
            name: name.to_string(),
            winners: winners.to_owned(),
            cards: cards.to_owned(),
        }
    } else {
        panic!("expected 2, failed on {:?}", parts);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sample_input() {
        assert_eq!(
            card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").points(),
            8
        );
        assert_eq!(
            card("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").points(),
            2
        );
        assert_eq!(
            card("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").points(),
            2
        );
        assert_eq!(
            card("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").points(),
            1
        );
        assert_eq!(
            card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").points(),
            0
        );
        assert_eq!(
            card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").points(),
            0
        );
    }
}
