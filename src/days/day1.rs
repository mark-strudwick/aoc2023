use crate::utils::{self, Solution};

#[derive(Default)]
pub struct Day1 {
    lines: Vec<String>,
}

impl Day1 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day1 {
    fn name(&self) -> (usize, usize) {
        (2023, 1)
    }

    fn parse_input(&mut self) {
        self.lines = utils::read_lines("./inputs/day1.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;

        for line in &self.lines {
            let numeric_chars: Vec<u32> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect();

            let first_num = numeric_chars.iter().nth(0).unwrap();
            let last_num = numeric_chars.iter().last().unwrap();

            let result = first_num * 10 + last_num;
            total += result;
        }

        vec![total.to_string()]
    }

    fn part2(&mut self) -> Vec<String> {
        for l in self.lines.iter_mut() {
            *l = str::replace(l, "one", "o1e");
            *l = str::replace(l, "two", "t2o");
            *l = str::replace(l, "three", "th3ee");
            *l = str::replace(l, "four", "fo4ur");
            *l = str::replace(l, "five", "fi5ve");
            *l = str::replace(l, "six", "s6x");
            *l = str::replace(l, "seven", "se7en");
            *l = str::replace(l, "eight", "ei8ght");
            *l = str::replace(l, "nine", "ni9ne");
        }

        let mut total = 0;

        for line in &self.lines {
            let numeric_chars: Vec<u32> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect();

            let first_num = numeric_chars.iter().nth(0);
            let last_num = numeric_chars.iter().last();

            if let (Some(first), Some(last)) = (first_num, last_num) {
                let result = format!("{:?}{:?}", first, last);
                total += result.parse::<u32>().unwrap();
            }
        }

        vec![total.to_string()]
    }
}
