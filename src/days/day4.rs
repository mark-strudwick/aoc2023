use crate::utils::{self, Solution};

#[derive(Default)]
pub struct Day4 {
    card_games: Vec<CardGame>,
}

impl Day4 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day4 {
    fn name(&self) -> (usize, usize) {
        (2023, 4)
    }

    fn parse_input(&mut self) {
        let lines = utils::read_lines("./inputs/day4.txt");

        for line in lines {
            let card_numbers_string = line.split_once(":").unwrap().1.split("|").nth(0).unwrap();
            let winning_numbers_string = line.split_once(":").unwrap().1.split("|").nth(1).unwrap();
            let card_game = CardGame::new(card_numbers_string, winning_numbers_string);
            self.card_games.push(card_game);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;

        for game in &self.card_games {
            if game.matches == 0 {
                continue;
            }
            let base: i32 = 2;
            let points = base.pow(game.matches as u32 - 1);
            total += points;
        }

        vec![total.to_string()]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;

        for i in 0..self.card_games.len() {
            let matches = self.card_games[i].matches;

            for j in i + 1..i + matches + 1 {
                self.card_games[j].add_copy_count();
            }

            for _c in 0..self.card_games[i].copies {
                for j in i + 1..i + matches + 1 {
                    self.card_games[j].add_copy_count();
                }
            }
        }

        for game in &self.card_games {
            total += game.copies + 1;
        }

        vec![total.to_string()]
    }
}

#[derive(Debug)]
struct CardGame {
    matches: usize,
    copies: usize,
}

impl CardGame {
    fn new(card_numbers_string: &str, winning_numbers_string: &str) -> Self {
        let card_numbers_split: Vec<&str> = card_numbers_string.split(" ").collect();
        let card_numbers: Vec<i64> = card_numbers_split
            .iter()
            .filter_map(|c| c.trim().parse().ok())
            .collect();

        let winning_numbers_split: Vec<&str> = winning_numbers_string.split(" ").collect();
        let winning_numbers: Vec<i64> = winning_numbers_split
            .iter()
            .filter_map(|c| c.trim().parse().ok())
            .collect();

        let matches = winning_numbers
            .iter()
            .filter(|&x| card_numbers.contains(x))
            .count();

        Self { matches, copies: 0 }
    }

    fn add_copy_count(&mut self) {
        self.copies += 1;
    }
}
