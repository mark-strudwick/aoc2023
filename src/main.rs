mod days;
mod utils;

use days::{day1::Day1, day2::Day2, day3::Day3, day4::Day4, day5::Day5, day6::Day6, day7::Day7};

use crate::utils::Solution;

fn main() {
    let mut day01 = Day1::new();
    let mut day02 = Day2::new();
    let mut day03 = Day3::new();
    let mut day04 = Day4::new();
    // let mut day05 = Day5::new();
    let mut day06 = Day6::new();
    let mut day07 = Day7::new();

    let days: Vec<&mut dyn Solution> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, // &mut day05
        &mut day06, &mut day07
    ];

    for d in days {
        utils::run_solution(d);
    }
}
