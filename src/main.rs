mod days;
mod utils;

use days::day1::Day1;
use days::day2::Day2;

use crate::utils::Solution;

fn main() {
    let mut day01 = Day1::new();
    let mut day02 = Day2::new();

    let days: Vec<&mut dyn Solution> = vec![&mut day01, &mut day02];

    for d in days {
        utils::run_solution(d);
    }
}
