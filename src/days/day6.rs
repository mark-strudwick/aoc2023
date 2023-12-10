use crate::utils::{self, Solution};

#[derive(Default)]
pub struct Day6 {
    races: Vec<Race>,
    part2_races: Vec<Race>,
}

impl Day6 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day6 {
    fn name(&self) -> (usize, usize) {
        (2023, 6)
    }

    fn parse_input(&mut self) {
        let lines = utils::read_lines("./inputs/day6.txt");
        let time_line: Vec<u64> = lines[0].split("Time:      ").collect::<Vec<_>>()[1]
            .split(" ")
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let distance_line: Vec<u64> = lines[1].split("Distance:  ").collect::<Vec<_>>()[1]
            .split(" ")
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        // loop through both vectors at the same time, and create races
        for (time, distance) in time_line.into_iter().zip(distance_line.into_iter()) {
            let race = Race::new(time, distance);
            self.races.push(race)
        }

        let lines = utils::read_lines("./inputs/day6.txt");
        let time_line_2: Vec<u64> = lines[0].split("Time:      ").collect::<Vec<_>>()[1]
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let time2: u64 = time_line_2.iter().fold(0, |acc, &x| acc * 10 + x);

        let distance_line_2: Vec<u64> = lines[1].split("Distance:  ").collect::<Vec<_>>()[1]
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        let mut string_digit: String = "".to_string();
        for char in distance_line_2 {
            string_digit.push(char.to_string().parse::<char>().unwrap());
        }

        let distance2: u64 = string_digit.parse().unwrap();
        let part2_race = Race::new(time2, distance2);
        self.part2_races.push(part2_race);
    }

    fn part1(&mut self) -> Vec<String> {
        let total: u64 = self
            .races
            .iter()
            .map(|r| r.distances.len() as u64)
            .product();

        vec![total.to_string()]
    }

    fn part2(&mut self) -> Vec<String> {
        let total: u64 = self.part2_races[0].distances.len().try_into().unwrap();

        vec![total.to_string()]
    }
}

#[derive(Debug)]
struct Race {
    time_milliseconds: u64,
    distance_record_millimeters: u64,
    distances: Vec<u64>,
}

impl Race {
    fn new(time_milliseconds: u64, distance_record_millimeters: u64) -> Self {
        let mut distances: Vec<u64> = vec![];

        for button_hold in 0..time_milliseconds {
            let speed = button_hold;
            let time_remaining = time_milliseconds - button_hold;
            let distance = speed * time_remaining;

            if distance > distance_record_millimeters {
                distances.push(distance);
            }
        }

        Self {
            time_milliseconds,
            distance_record_millimeters,
            distances,
        }
    }
}
