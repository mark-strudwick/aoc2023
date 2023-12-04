use crate::utils::{self, Solution};

#[derive(Default)]
pub struct Day4 {}

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
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;

        vec![total.to_string()]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;

        vec![total.to_string()]
    }
}
