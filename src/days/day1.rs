use anyhow::Result;
use std::fs;

pub fn part1() -> Result<String> {
    let file_lines: Vec<String> = fs::read_to_string("./inputs/day1.txt")
        .expect("Could not read file 'day1'")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut total = 0;
    for l in file_lines.iter() {
        let mut first_num = 0;
        let mut last_num = 0;
        for c in l.chars() {
            if c.is_digit(10) && first_num == 0 {
                first_num = c.to_digit(10).unwrap();
            }
        }
        for c in l.chars().rev() {
            if c.is_digit(10) && last_num == 0 {
                last_num = c.to_digit(10).unwrap();
            }
        }
        let result = format!("{}{}", first_num.to_string(), last_num.to_string());
        total += result.parse::<i32>().unwrap();
    }

    Ok(total.to_string())
}

pub fn part2() -> Result<String> {
    let mut file_lines: Vec<String> = fs::read_to_string("./inputs/day1.txt")
        .expect("Could not read file 'day1'")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    for l in file_lines.iter_mut() {
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
    for l in file_lines.iter() {
        let mut first_num = 0;
        let mut last_num = 0;
        for c in l.chars() {
            if c.is_digit(10) && first_num == 0 {
                first_num = c.to_digit(10).unwrap();
            }
        }
        for c in l.chars().rev() {
            if c.is_digit(10) && last_num == 0 {
                last_num = c.to_digit(10).unwrap();
            }
        }
        let result = format!("{}{}", first_num.to_string(), last_num.to_string());
        total += result.parse::<i32>().unwrap();
    }

    Ok(total.to_string())
}
