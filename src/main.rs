mod days;
mod utils;

use days::{day1::Day1, day2::Day2, day3::Day3};

use crate::utils::Solution;

fn main() {
    let mut day01 = Day1::new();
    let mut day02 = Day2::new();
    let mut day03 = Day3::new();

    let days: Vec<&mut dyn Solution> = vec![&mut day01, &mut day02, &mut day03];

    for d in days {
        utils::run_solution(d);
    }
}
