use std::collections::HashSet;

use crate::utils::{self, Solution};

#[derive(Default)]
pub struct Day3 {
    engine_parts: Vec<EnginePart>,
    symbols: HashSet<(i64, i64)>,
    gears: HashSet<(i64, i64)>,
}

impl Day3 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day3 {
    fn name(&self) -> (usize, usize) {
        (2023, 3)
    }

    fn parse_input(&mut self) {
        let lines = utils::read_lines("./inputs/day3.txt");

        let mut cur_number: Option<EnginePart> = None;
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch.is_ascii_digit() {
                    if let Some(ref mut num) = cur_number {
                        num.add_point(ch, row as i64, col as i64);
                    } else {
                        cur_number = Some(EnginePart::new(ch, row as i64, col as i64));
                    }
                } else {
                    if let Some(num) = cur_number.take() {
                        self.engine_parts.push(num);
                    }
                    if ch != '.' {
                        self.symbols.insert((row as i64, col as i64));
                        if ch == '*' {
                            self.gears.insert((row as i64, col as i64));
                        }
                    }
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;

        for num in &self.engine_parts {
            if num.is_adjacent_to_symbol(&self.symbols) {
                total += num.value;
            }
        }

        vec![total.to_string()]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;

        'skip_gear: for gear in &self.gears {
            let mut gear_matches = Vec::new();

            for part in &self.engine_parts {
                if part.points.contains(gear) {
                    if gear_matches.len() == 2 {
                        continue 'skip_gear;
                    }
                    gear_matches.push(part.value);
                }
            }
            if gear_matches.len() == 2 {
                total += gear_matches[0] * gear_matches[1];
            }
        }

        vec![total.to_string()]
    }
}

struct EnginePart {
    value: i64,
    points: HashSet<(i64, i64)>,
}

impl EnginePart {
    fn new(char: char, row: i64, col: i64) -> Self {
        let points = HashSet::from([
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1), // left hand side
            (row - 1, col),
            (row + 1, col), // above and below
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1), // right hand side
        ]);

        Self {
            value: char.to_digit(10).unwrap() as i64,
            points,
        }
    }

    fn add_point(&mut self, char: char, row: i64, col: i64) {
        self.value = self.value * 10 + (char.to_digit(10).unwrap() as i64);
        self.points
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }

    fn is_adjacent_to_symbol(&self, symbols: &HashSet<(i64, i64)>) -> bool {
        self.points.intersection(&symbols).next().is_some()
    }
}
