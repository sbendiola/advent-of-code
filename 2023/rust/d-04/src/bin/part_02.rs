use regex::Regex;
use std::ops::Add;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn main() {
    let cards = fs::read_to_string("resources/input")
        .expect("failed opening file")
        .lines()
        .map(|line| card(line))
        .collect::<Vec<Card>>();

    let sum = scratch_cards(&cards);
    println!("part {} {:?}", file!().to_string(), sum);
}

fn scratch_cards(cards: &Vec<Card>) -> u32 {
    let mut copies = vec![1u32; cards.len()];
    for i in 0..cards.len() {
        let card = cards.get(i).unwrap();
        let mut count = card.count();
        let day_copies = copies[i];
        let mut next: usize = i + 1;
        println!("card {}  won {:?}", card.name, card.count());
        loop {
            if count == 0 {
                break;
            }
            if let None = copies.get(next) {
                break;
            } else {
                let before = copies[next];
                copies[next] += day_copies;
                println!("updated card {} {} -> {}", next + 1, before, copies[next]);
            }
            next += 1;
            count -= 1;
        }
    }
    copies.iter().sum()
}
#[derive(Debug, Eq, PartialEq, Clone, Hash, PartialOrd, Ord)]
pub struct Day(u32);

impl Add<u32> for Day {
    type Output = Self;
    fn add(self, other: u32) -> Self {
        Day(self.0 + other)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Card {
    line: String,
    name: u32,
    winners: HashSet<u8>,
    cards: HashSet<u8>,
    copies: HashMap<Day, u32>,
}

impl Card {
    fn count(&self) -> u32 {
        self.cards.intersection(&self.winners).count() as u32
    }
}

fn card(text: &str) -> Card {
    let re = Regex::new(r"^Card\s*(?P<name>\d+): (?P<winners>[^\|]+)\|(?P<cards>.*)$");

    let matched = re
        .expect("regex failed")
        .captures(text)
        .expect(format!("did not match {:?}", text).as_str());

    let name = matched
        .name("name")
        .unwrap()
        .as_str()
        .trim()
        .parse::<u32>()
        .unwrap();

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
        line: text.to_string(),
        name: name,
        winners: winners,
        cards: cards,
        copies: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE_INPUT: &str = r#"
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#;
    #[test]
    fn test_sample_input3() {
        let cards: Vec<Card> = SAMPLE_INPUT
            .lines()
            .map(|f| f.trim())
            .filter(|line| !line.is_empty())
            .map(|line| card(line))
            .collect();

        let total = scratch_cards(&cards);
        println!("part {} {:?}", file!().to_string(), total);
        assert_eq!(total, 30)
    }
}
