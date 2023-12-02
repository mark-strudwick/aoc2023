use anyhow::Result;
use std::fs;

pub fn part1() -> Result<String> {
    let lines: Vec<String> = fs::read_to_string("./inputs/day1.txt")
        .expect("Could not read file 'day1'")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut total = 0;

    for line in lines {
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

    Ok(total.to_string())
}

pub fn part2() -> Result<String> {
    let mut lines: Vec<String> = fs::read_to_string("./inputs/day1.txt")
        .expect("Could not read file 'day1'")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    for l in lines.iter_mut() {
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

    for line in lines {
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

    Ok(total.to_string())
}
