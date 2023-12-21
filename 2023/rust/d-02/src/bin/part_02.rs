const DEBUG: bool = false;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct Drawing {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Game {
    id: usize,
    draws: Vec<Drawing>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct Settings {
    red: usize,
    green: usize,
    blue: usize,
}

impl Settings {
    fn add(self, color: &str, count: usize) -> Self {
        let mut x = self;

        match color {
            "red" => x.red += count,
            "blue" => x.blue += count,
            "green" => x.green += count,
            _ => panic!("unknown color"),
        };
        x
    }
}
use regex::Regex;

fn game_from_text(line: &str) -> Game {
    let draw_text = Regex::new(r"^Game (\d+): (.*)")
        .expect("failed to get draw regex")
        .captures(line)
        .expect("failed to find draw");

    let game_id = draw_text
        .get(1)
        .expect("failed to extract group id")
        .as_str()
        .parse::<usize>()
        .expect(format!("failed to parse id {:?}", line).as_str());

    let draws: Vec<Drawing> = draw_text
        .get(2)
        .expect("failed to extract group draw")
        .as_str()
        .split(";")
        .map(|draw_line| to_drawing(draw_line))
        .collect();

    Game {
        id: game_id,
        draws: draws,
    }
}

fn to_drawing(draw_line: &str) -> Drawing {
    let counts =
        draw_line
            .split(",")
            .map(str::trim)
            .fold(Settings::default(), |settings, color_count| {
                let color_count_match = Regex::new(r"^(?P<count>\d+) (?P<color>red|blue|green)")
                    .expect("failed to get color counts")
                    .captures(color_count)
                    .expect(format!("failed to extract color counts {:?}", color_count).as_str());

                match color_count_match
                    .name("count")
                    .zip(color_count_match.name("color"))
                {
                    Some((count, color)) => {
                        settings.add(color.as_str(), count.as_str().parse::<usize>().unwrap())
                    }
                    None => panic!("failed to extract color counts {:?}", color_count),
                }
            });

    Drawing {
        red: counts.red,
        green: counts.green,
        blue: counts.blue,
    }
}

use std::fs;
pub fn main() {
    let initial: usize = 0;
    let power_count = fs::read_to_string("resources/input")
        .expect("failed opening file")
        .lines()
        .fold(initial, |acc, line| acc + power(game_from_text(line)));
    assert_eq!(power_count, 72227);
    println!("day2 part 2 power_count:{:?}", power_count);
}

fn power(game: Game) -> usize {
    let result = game
        .draws
        .iter()
        .fold(Settings::default(), |mut acc, draw| {
            acc.red = std::cmp::max(acc.red, draw.red);
            acc.blue = std::cmp::max(acc.blue, draw.blue);
            acc.green = std::cmp::max(acc.green, draw.green);
            acc
        });
    let pow = result.red * result.blue * result.green;

    if DEBUG {
        println!(
            "{} r:{} g:{} b:{} result:{:?}",
            game.id, result.red, result.green, result.blue, pow
        );
    }

    pow
}

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
extern crate quickcheck_macros;

use std::fmt::Debug;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
impl Arbitrary for Game {
    fn arbitrary(g: &mut Gen) -> Self {
        Game {
            id: usize::arbitrary(g),
            draws: Vec::arbitrary(g),
        }
    }
}
#[cfg(test)]
impl Arbitrary for Drawing {
    fn arbitrary(g: &mut Gen) -> Self {
        let red = usize::arbitrary(g);
        let green = usize::arbitrary(g);
        let blue = usize::arbitrary(g);
        Drawing { red, green, blue }
    }
}
#[cfg(test)]
impl Arbitrary for Settings {
    fn arbitrary(g: &mut Gen) -> Self {
        let red = usize::arbitrary(g);
        let green = usize::arbitrary(g);
        let blue = usize::arbitrary(g);
        Settings { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_game_from_text() {
        let expected_values: HashMap<&str, Game> = [
            (
                ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
                Game {
                    id: 1,
                    draws: vec![
                        Drawing {
                            red: 4,
                            green: 0,
                            blue: 3,
                        },
                        Drawing {
                            red: 1,
                            green: 2,
                            blue: 6,
                        },
                        Drawing {
                            red: 0,
                            green: 2,
                            blue: 0,
                        },
                    ],
                },
            ),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                Game {
                    id: 2,
                    draws: vec![
                        Drawing {
                            red: 0,
                            green: 2,
                            blue: 1,
                        },
                        Drawing {
                            red: 1,
                            green: 3,
                            blue: 4,
                        },
                        Drawing {
                            red: 0,
                            green: 1,
                            blue: 1,
                        },
                    ],
                },
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                Game {
                    id: 3,
                    draws: vec![
                        Drawing {
                            red: 20,
                            green: 8,
                            blue: 6,
                        },
                        Drawing {
                            red: 4,
                            green: 13,
                            blue: 5,
                        },
                        Drawing {
                            red: 1,
                            green: 5,
                            blue: 0,
                        },
                    ],
                },
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                Game {
                    id: 4,
                    draws: vec![
                        Drawing {
                            red: 3,
                            green: 1,
                            blue: 6,
                        },
                        Drawing {
                            red: 6,
                            green: 3,
                            blue: 0,
                        },
                        Drawing {
                            red: 14,
                            green: 3,
                            blue: 15,
                        },
                    ],
                },
            ),
            (
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                Game {
                    id: 5,
                    draws: vec![
                        Drawing {
                            red: 6,
                            green: 3,
                            blue: 1,
                        },
                        Drawing {
                            red: 1,
                            green: 2,
                            blue: 2,
                        },
                    ],
                },
            ),
        ]
        .iter()
        .cloned()
        .collect();

        expected_values.iter().for_each(|(text, game_expected)| {
            let game_actual = game_from_text(text);
            assert_eq!(game_actual, game_expected.clone());
        });
    }
}
