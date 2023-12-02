use crate::utils::{self, Solution};
use std::fs;

const ALLOWED_NUM_RED: u32 = 12;
const ALLOWED_NUM_GREEN: u32 = 13;
const ALLOWED_NUM_BLUE: u32 = 14;

#[derive(Debug, PartialEq)]
struct BagGame {
    id: u32,
    sets: Vec<Vec<(String, u32)>>,
}

#[derive(Default)]
pub struct Day2 {
    lines: Vec<String>,
}

impl Day2 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day2 {
    fn name(&self) -> (usize, usize) {
        (2023, 2)
    }

    fn parse_input(&mut self) {
        self.lines = utils::read_lines("./inputs/day2.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let games = format_input_to_games(&self.lines);

        let mut total = 0;

        for game in games {
            let mut allowed = true;

            for set in game.sets {
                for (color, count) in set {
                    match color.as_str() {
                        "red" => {
                            if count > ALLOWED_NUM_RED {
                                allowed = false;
                            }
                        }
                        "green" => {
                            if count > ALLOWED_NUM_GREEN {
                                allowed = false;
                            }
                        }
                        "blue" => {
                            if count > ALLOWED_NUM_BLUE {
                                allowed = false;
                            }
                        }
                        _ => println!("Unknown color: {}", color),
                    }
                }
            }

            if allowed {
                total += game.id;
            }
        }

        vec![total.to_string()]
    }

    fn part2(&mut self) -> Vec<String> {
        let lines: Vec<String> = fs::read_to_string("./inputs/day2.txt")
            .expect("Could not read file 'day2'")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        let games = format_input_to_games(&lines);

        let mut total = 0;

        for game in games {
            let mut fewest_reds = 0;
            let mut fewest_greens = 0;
            let mut fewest_blues = 0;
            for set in game.sets {
                for (color, count) in set {
                    match color.as_str() {
                        "red" => {
                            if count > fewest_reds {
                                fewest_reds = count;
                            }
                        }
                        "green" => {
                            if count > fewest_greens {
                                fewest_greens = count;
                            }
                        }
                        "blue" => {
                            if count > fewest_blues {
                                fewest_blues = count;
                            }
                        }
                        _ => println!("Unknown color: {}", color),
                    }
                }
            }

            let power = fewest_reds * fewest_greens * fewest_blues;
            total += power;
        }

        vec![total.to_string()]
    }
}

fn format_input_to_games(input: &Vec<String>) -> Vec<BagGame> {
    let mut games: Vec<BagGame> = Vec::new();

    for line in input {
        if let Some((id_part, cubes_part)) = line.split_once(":") {
            let id = id_part.split_once(" ").unwrap().1;
            let game_sets: Vec<&str> = cubes_part.split(";").collect();

            let mut bag_game = BagGame {
                id: id.parse::<u32>().unwrap(),
                sets: Vec::new(),
            };

            for game in game_sets {
                let cubes: Vec<&str> = game.split(",").collect();
                // [" 5 blue", " 9 green", " 1 red"]
                let color_count: Vec<(String, u32)> = cubes
                    .iter()
                    .map(|&s| {
                        let mut parts = s.split_whitespace();
                        let count = parts.next().unwrap().trim().parse().unwrap();
                        let color = parts.next().unwrap().trim().to_string();
                        (color, count)
                    })
                    .collect();

                bag_game.sets.push(color_count);
            }

            games.push(bag_game);
        }
    }

    games
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_input_to_baggames() {
        let input: Vec<String> = vec![
        "Game 1: 9 red, 2 green, 13 blue; 10 blue, 2 green, 13 red; 8 blue, 3 red, 6 green; 5 green, 2 red, 1 blue".to_string(),
        "Game 2: 2 green, 2 blue, 16 red; 14 red; 13 red, 13 green, 2 blue; 7 red, 7 green, 2 blue".to_string(),
        "Game 3: 6 red, 4 green, 7 blue; 7 blue, 9 red, 3 green; 2 red, 4 green; 6 red, 2 blue; 7 blue, 9 red, 5 green".to_string(),
        ];

        let games = format_input_to_games(&input);

        let expected = [
            BagGame {
                id: 1,
                sets: vec![
                    vec![
                        ("red".to_string(), 9),
                        ("green".to_string(), 2),
                        ("blue".to_string(), 13),
                    ],
                    vec![
                        ("blue".to_string(), 10),
                        ("green".to_string(), 2),
                        ("red".to_string(), 13),
                    ],
                    vec![
                        ("blue".to_string(), 8),
                        ("red".to_string(), 3),
                        ("green".to_string(), 6),
                    ],
                    vec![
                        ("green".to_string(), 5),
                        ("red".to_string(), 2),
                        ("blue".to_string(), 1),
                    ],
                ],
            },
            BagGame {
                id: 2,
                sets: vec![
                    vec![
                        ("green".to_string(), 2),
                        ("blue".to_string(), 2),
                        ("red".to_string(), 16),
                    ],
                    vec![("red".to_string(), 14)],
                    vec![
                        ("red".to_string(), 13),
                        ("green".to_string(), 13),
                        ("blue".to_string(), 2),
                    ],
                    vec![
                        ("red".to_string(), 7),
                        ("green".to_string(), 7),
                        ("blue".to_string(), 2),
                    ],
                ],
            },
            BagGame {
                id: 3,
                sets: vec![
                    vec![
                        ("red".to_string(), 6),
                        ("green".to_string(), 4),
                        ("blue".to_string(), 7),
                    ],
                    vec![
                        ("blue".to_string(), 7),
                        ("red".to_string(), 9),
                        ("green".to_string(), 3),
                    ],
                    vec![("red".to_string(), 2), ("green".to_string(), 4)],
                    vec![("red".to_string(), 6), ("blue".to_string(), 2)],
                    vec![
                        ("blue".to_string(), 7),
                        ("red".to_string(), 9),
                        ("green".to_string(), 5),
                    ],
                ],
            },
        ];
        assert_eq!(games, expected);
    }
}
