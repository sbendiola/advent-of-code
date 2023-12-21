use regex::Regex;
use std::fs;

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

fn game_from_text(line: &str) -> Game {
    let draw_text = Regex::new(r"^Game (?P<id>\d+): (?P<game_line>.*)")
        .expect("failed to get draw regex")
        .captures(line)
        .expect("failed to find draw");

    match draw_text.name("id").zip(draw_text.name("game_line")) {
        Some((id, game_line)) => {
            let game_id = id
                .as_str()
                .parse::<usize>()
                .expect(format!("failed to parse id {:?}", line).as_str());

            let drawings = game_line.as_str().split(";").map(|draw_line| {
                let counts = draw_line
                    .split(",")
                    .map(str::trim)
                    .fold(Settings::default(), |acc, color_count| {
                        counts(acc, color_count)
                    });

                Drawing {
                    red: counts.red,
                    green: counts.green,
                    blue: counts.blue,
                }
            });

            Game {
                id: game_id,
                draws: drawings.collect(),
            }
        }
        None => panic!("failed to get id and game_line"),
    }
}

fn counts(settings: Settings, color_count: &str) -> Settings {
    let color_count_match = Regex::new(r"^(?P<count>\d+) (?P<color>red|blue|green)")
        .expect("failed to get color counts")
        .captures(color_count)
        .expect(format!("failed to extract color counts {:?}", color_count).as_str());

    match color_count_match
        .name("count")
        .zip(color_count_match.name("color"))
    {
        Some((count, color)) => {
            let count = count
                .as_str()
                .trim()
                .parse::<usize>()
                .expect("failed to parse count");
            let color_text = color.as_str().trim();
            settings.add(color_text, count)
        }
        None => panic!("failed to get count and color"),
    }
}

pub fn main() {
    const PARAMS: Settings = Settings {
        red: 12,
        green: 13,
        blue: 14,
    };

    let possible_count: usize = fs::read_to_string("resources/input")
        .expect("failed opening file")
        .lines()
        .fold(0, |acc, line| {
            let g = game_from_text(line);
            let id = g.id;
            acc + if possible(&g, &PARAMS) { id } else { 0 }
        });
    assert_eq!(possible_count, 2716);
    println!("day1 part 1 {:?}", possible_count);
}

fn possible(game: &Game, settings: &Settings) -> bool {
    game.draws.iter().all(|draw| {
        draw.red <= settings.red && draw.green <= settings.green && draw.blue <= settings.blue
    })
}

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
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
    use std::collections::HashMap;

    use super::*;
    #[quickcheck]
    fn prop_possible(game: Game, settings: Settings) -> bool {
        game.draws.iter().all(|draw| {
            draw.red <= settings.red && draw.green <= settings.green && draw.blue <= settings.blue
        }) == possible(&game, &settings)
    }
}
