use anyhow::Result;
use std::fs;

// Allowed cubes:
// Red: 12
// Green: 13
// Blue: 14

// Example line:
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
const ALLOWED_RED: u32 = 12;
const ALLOWED_GREEN: u32 = 13;
const ALLOWED_BLUE: u32 = 14;

#[derive(Debug, PartialEq)]
struct BagGame {
    id: u32,
    sets: Vec<Vec<(String, u32)>>,
}

pub fn part1() -> Result<String> {
    let lines: Vec<String> = fs::read_to_string("./inputs/day2.txt")
        .expect("Could not read file 'day1'")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let games = format_input_to_games(lines);

    let mut total = 0;

    for game in games {
        let mut allowed = true;

        for set in game.sets {
            for (color, count) in set {
                match color.as_str() {
                    "red" => {
                        if count > ALLOWED_RED {
                            allowed = false;
                        }
                    }
                    "green" => {
                        if count > ALLOWED_GREEN {
                            allowed = false;
                        }
                    }
                    "blue" => {
                        if count > ALLOWED_BLUE {
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

    Ok(total.to_string())
}

fn format_input_to_games(input: Vec<String>) -> Vec<BagGame> {
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
        let input: Vec<String> = [
        "Game 1: 9 red, 2 green, 13 blue; 10 blue, 2 green, 13 red; 8 blue, 3 red, 6 green; 5 green, 2 red, 1 blue".to_string(),
        "Game 2: 2 green, 2 blue, 16 red; 14 red; 13 red, 13 green, 2 blue; 7 red, 7 green, 2 blue".to_string(),
        "Game 3: 6 red, 4 green, 7 blue; 7 blue, 9 red, 3 green; 2 red, 4 green; 6 red, 2 blue; 7 blue, 9 red, 5 green".to_string(),
        ].to_vec();

        let games = format_input_to_games(input);

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