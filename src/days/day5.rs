use crate::utils::{self, Solution};

#[derive(Default)]
pub struct Day5 {}

impl Day5 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day5 {
    fn name(&self) -> (usize, usize) {
        (2023, 5)
    }

    fn parse_input(&mut self) {
        utils::read_lines("./inputs/day5.txt");
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
