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
            // Sort this out!
            let card_number_string = line.split_once(":").unwrap().0;
            let card_numbers_string = line.split_once(":").unwrap().1.split("|").nth(0).unwrap();
            let winning_numbers_string = line.split_once(":").unwrap().1.split("|").nth(1).unwrap();
            let card_game = CardGame::new(
                card_number_string,
                card_numbers_string,
                winning_numbers_string,
            );
            self.card_games.push(card_game);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;

        for game in &self.card_games {
            let matching_numbers = game
                .winning_numbers
                .iter()
                .filter(|&x| game.card_numbers.contains(x))
                .count();

            if matching_numbers == 0 {
                continue;
            }
            let base: i32 = 2;
            let points = base.pow(matching_numbers as u32 - 1);
            total += points;
        }

        vec![total.to_string()]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;

        vec![total.to_string()]
    }
}

#[derive(Debug)]
struct CardGame {
    card_number: i64,
    card_numbers: Vec<i64>,
    winning_numbers: Vec<i64>,
}

impl CardGame {
    fn new(
        card_number_string: &str,
        card_numbers_string: &str,
        winning_numbers_string: &str,
    ) -> Self {
        let card_number = card_number_string
            .split_once(" ")
            .unwrap()
            .1
            .trim()
            .parse()
            .unwrap();

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

        Self {
            card_number,
            card_numbers,
            winning_numbers,
        }
    }
}
