mod days;
mod utils;

use days::day1::Day1;

use crate::utils::Solution;

fn main() {
    let mut day01 = Day1::new();

    let days: Vec<&mut dyn Solution> = vec![&mut day01];

    for d in days {
        utils::run_solution(d);
    }

    let day2a = days::day2::part1();
    println!("Day2: a: {:?}", day2a.unwrap());

    let day2b = days::day2::part2();
    println!("Day2: b: {:?}", day2b.unwrap());
}
