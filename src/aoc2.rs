use crate::aoc_traits::{Solution, SolveAdvent};
use std::{fs, num::ParseIntError};

fn read_input(aoc: &Aoc2) -> String {
    fs::read_to_string(format!("inputs/{}.txt", aoc.name)).expect("Could not read input.")
}

pub struct Aoc2 {
    name: String,
}

impl Aoc2 {
    pub fn new() -> Aoc2 {
        Aoc2 {
            name: "aoc2".to_string(),
        }
    }
}

impl SolveAdvent for Aoc2 {
    fn compute_solution1(&self) -> Solution {
        let input = read_input(self);
        let game_bag = GameBag::from("red: 12, green: 13, blue: 14");

        let games = input
            .split('\n')
            .filter(|game| game.len() != 0)
            .map(|game| {
                let game_data = game.split_once(':').expect("Could not split game info.");

                (
                    game_data
                        .0
                        .trim()
                        .split_once(' ')
                        .expect("Could not split game name | game id.")
                        .1
                        .parse::<i32>()
                        .expect("Could not parse game ID."),
                    game_data.1,
                )
            })
            .collect::<Vec<(i32, &str)>>();

        let mut valid_games = 0;

        for game in games {
            let game_sets = game
                .1
                .split(';')
                .filter(|game_set| game_set.len() != 0)
                .map(|game_set| {
                    GameSet::try_from(game_set).expect("Could not convert str to gameset.")
                })
                .collect::<Vec<GameSet>>();

            let valid_game = game_sets
                .iter()
                .map(|game_set| {
                    let result = game_bag.is_game_set_possible(game_set);
                    result
                })
                .fold(true, |acc, val| acc && val);

            if valid_game {
                valid_games += game.0;
            }
        }

        Solution::new(&self.name, valid_games.to_string())
    }
    fn compute_solution2(&self) -> Solution {
        let input = read_input(self);

        let games = input
            .split('\n')
            .filter(|game| game.len() != 0)
            .map(|game| game.split_once(':').expect("Could not split game info.").1)
            .collect::<Vec<&str>>();

        let mut power_set = 0;

        for game in games {
            let game_sets = game
                .split(';')
                .filter(|game_set| game_set.len() != 0)
                .map(|game_set| {
                    GameSet::try_from(game_set).expect("Could not convert str to gameset.")
                })
                .collect::<Vec<GameSet>>();

            let required_colors: (i32, i32, i32) =
                game_sets.iter().fold((0, 0, 0), |mut acc, game_set| {
                    if game_set.red as i32 >= acc.0 {
                        acc.0 = game_set.red as i32;
                    }
                    if game_set.green as i32 >= acc.1 {
                        acc.1 = game_set.green as i32;
                    }
                    if game_set.blue as i32 >= acc.2 {
                        acc.2 = game_set.blue as i32;
                    }

                    acc
                });

            power_set += required_colors.0 * required_colors.1 * required_colors.2;
        }

        Solution::new(&self.name, power_set.to_string())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GameBag {
    red: u8,
    green: u8,
    blue: u8,
}

impl GameBag {
    pub fn new(red: u8, green: u8, blue: u8) -> GameBag {
        GameBag { red, green, blue }
    }

    pub fn is_game_set_possible(&self, game_set: &GameSet) -> bool {
        self.red >= game_set.red && self.green >= game_set.green && self.blue >= game_set.blue
    }
}

impl Default for GameBag {
    fn default() -> Self {
        GameBag::new(1, 1, 1)
    }
}

impl From<&str> for GameBag {
    fn from(value: &str) -> Self {
        // red: u8, green: u8, blue: u8
        let colors = value
            .split(',')
            .map(|color| {
                let color_value = color
                    .split_once(':')
                    .expect("Expected a tuple of size two.")
                    .1
                    .trim()
                    .parse::<u8>()
                    .expect("Could not parse value to u8.");
                color_value
            })
            .collect::<Vec<u8>>();

        if colors.len() != 3 {
            return GameBag::default();
        }

        GameBag::new(colors[0], colors[1], colors[2])
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GameSet {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
enum GameSetError {
    EmptyInput,
    MalformedInput,
    ParseInt(ParseIntError),
}

impl GameSet {
    pub fn new(red: u8, green: u8, blue: u8) -> GameSet {
        GameSet { red, green, blue }
    }
}

impl TryFrom<&str> for GameSet {
    type Error = GameSetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "" {
            return Err(GameSetError::EmptyInput);
        }

        let set_values = value
            .split(',')
            .map(|element| {
                let val_color_pair = element
                    .trim()
                    .split_once(' ')
                    .ok_or_else(|| GameSetError::MalformedInput)
                    .and_then(|element| {
                        let value = element
                            .0
                            .trim()
                            .parse::<u8>()
                            .map_err(|e| GameSetError::ParseInt(e))?;

                        let color = element.1.trim();
                        if color != "red" && color != "green" && color != "blue" {
                            return Err(GameSetError::MalformedInput);
                        }

                        Ok((value, color))
                    });

                val_color_pair
            })
            .collect::<Result<Vec<(u8, &str)>, GameSetError>>()?;

        let mut red = 0u8;
        let mut green = 0u8;
        let mut blue = 0u8;

        for (value, name) in set_values {
            if name == "red" {
                red = value;
            }
            if name == "green" {
                green = value;
            }
            if name == "blue" {
                blue = value;
            }
        }

        Ok(GameSet::new(red, green, blue))
    }
}
