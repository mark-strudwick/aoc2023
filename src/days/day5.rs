use std::ops::RangeInclusive;

use crate::utils::{self, Solution};

#[derive(Default)]
pub struct Day5 {
    seeds: Vec<u32>,
    seed_to_soil_mapping: SeedToSoil,
    soil_to_fertilizer_mapping: SoilToFertilizer,
    fertilizer_to_water_mapping: FertilizerToWater,
    water_to_light_mapping: WaterToLight,
    light_to_temperature_mapping: LightToTemperature,
    temperature_to_humidity_mapping: TemperatureToHumidity,
    humidity_to_location_mapping: HumidityToLocation,
}

impl Day5 {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
struct SourceDestMapping {
    source_range: RangeInclusive<u32>,
    dest_range: RangeInclusive<u32>,
}

#[derive(Debug, Default)]
struct SeedToSoil {
    mappings: Vec<SourceDestMapping>,
}

#[derive(Debug, Default)]
struct SoilToFertilizer {
    mappings: Vec<SourceDestMapping>,
}

#[derive(Debug, Default)]
struct FertilizerToWater {
    mappings: Vec<SourceDestMapping>,
}

#[derive(Debug, Default)]
struct WaterToLight {
    mappings: Vec<SourceDestMapping>,
}

#[derive(Debug, Default)]
struct LightToTemperature {
    mappings: Vec<SourceDestMapping>,
}

#[derive(Debug, Default)]
struct TemperatureToHumidity {
    mappings: Vec<SourceDestMapping>,
}

#[derive(Debug, Default)]
struct HumidityToLocation {
    mappings: Vec<SourceDestMapping>,
}

impl Solution for Day5 {
    fn name(&self) -> (usize, usize) {
        (2023, 5)
    }

    fn parse_input(&mut self) {
        let lines = utils::read_lines("./inputs/day5.txt");
        let seed_to_soil_index = lines
            .iter()
            .position(|x| x.contains("seed-to-soil map:"))
            .unwrap();
        let soil_to_fertilizer_index = lines
            .iter()
            .position(|x| x.contains("soil-to-fertilizer map:"))
            .unwrap();
        let fertilizer_to_water_index = lines
            .iter()
            .position(|x| x.contains("fertilizer-to-water map:"))
            .unwrap();
        let water_to_light_index = lines
            .iter()
            .position(|x| x.contains("water-to-light map:"))
            .unwrap();
        let light_to_temperature_index = lines
            .iter()
            .position(|x| x.contains("light-to-temperature map:"))
            .unwrap();
        let temperature_to_humidity_index = lines
            .iter()
            .position(|x| x.contains("temperature-to-humidity map:"))
            .unwrap();
        let humidity_to_location_index = lines
            .iter()
            .position(|x| x.contains("humidity-to-location map:"))
            .unwrap();

        let seeds: Vec<u32> = lines[0..seed_to_soil_index][0]
            .split_once(": ")
            .unwrap()
            .1
            .split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        self.seeds = seeds;

        let seed_to_soil = &lines[seed_to_soil_index + 1..soil_to_fertilizer_index];
        let seed_to_soil_parsed = seed_to_soil
            .iter()
            .map(|x| {
                let mut split = x.split(" ");
                let dest = split.next().unwrap().parse::<u32>().unwrap();
                let source = split.next().unwrap().parse::<u32>().unwrap();
                let range_length = split.next().unwrap().parse::<u32>().unwrap();
                (dest, source, range_length)
            })
            .collect::<Vec<(u32, u32, u32)>>();
        seed_to_soil_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let source_range = RangeInclusive::new(source, source + range_length - 1);
            let dest_range = RangeInclusive::new(dest, dest + range_length - 1);
            let mapping = SourceDestMapping {
                source_range,
                dest_range,
            };

            self.seed_to_soil_mapping.mappings.push(mapping);
        });

        let soil_to_fertilizer = &lines[soil_to_fertilizer_index + 1..fertilizer_to_water_index];
        let soil_to_fertilizer_parsed = soil_to_fertilizer
            .iter()
            .map(|x| {
                let mut split = x.split(" ");
                let dest = split.next().unwrap().parse::<u32>().unwrap();
                let source = split.next().unwrap().parse::<u32>().unwrap();
                let range_length = split.next().unwrap().parse::<u32>().unwrap();
                (dest, source, range_length)
            })
            .collect::<Vec<(u32, u32, u32)>>();
        soil_to_fertilizer_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let source_range = RangeInclusive::new(source, source + range_length - 1);
            let dest_range = RangeInclusive::new(dest, dest + range_length - 1);
            let mapping = SourceDestMapping {
                source_range,
                dest_range,
            };

            self.soil_to_fertilizer_mapping.mappings.push(mapping);
        });

        let fertilizer_to_water = &lines[fertilizer_to_water_index + 1..water_to_light_index];
        let fertilizer_to_water_parsed = fertilizer_to_water
            .iter()
            .map(|x| {
                let mut split = x.split(" ");
                let dest = split.next().unwrap().parse::<u32>().unwrap();
                let source = split.next().unwrap().parse::<u32>().unwrap();
                let range_length = split.next().unwrap().parse::<u32>().unwrap();
                (dest, source, range_length)
            })
            .collect::<Vec<(u32, u32, u32)>>();
        fertilizer_to_water_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let source_range = RangeInclusive::new(source, source + range_length - 1);
            let dest_range = RangeInclusive::new(dest, dest + range_length - 1);
            let mapping = SourceDestMapping {
                source_range,
                dest_range,
            };

            self.fertilizer_to_water_mapping.mappings.push(mapping);
        });

        let water_to_light = &lines[water_to_light_index + 1..light_to_temperature_index];
        let water_to_light_parsed = water_to_light
            .iter()
            .map(|x| {
                let mut split = x.split(" ");
                let dest = split.next().unwrap().parse::<u32>().unwrap();
                let source = split.next().unwrap().parse::<u32>().unwrap();
                let range_length = split.next().unwrap().parse::<u32>().unwrap();
                (dest, source, range_length)
            })
            .collect::<Vec<(u32, u32, u32)>>();
        water_to_light_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let source_range = RangeInclusive::new(source, source + range_length - 1);
            let dest_range = RangeInclusive::new(dest, dest + range_length - 1);
            let mapping = SourceDestMapping {
                source_range,
                dest_range,
            };

            self.water_to_light_mapping.mappings.push(mapping);
        });

        let light_to_temperature =
            &lines[light_to_temperature_index + 1..temperature_to_humidity_index];
        let light_to_temperature_parsed = light_to_temperature
            .iter()
            .map(|x| {
                let mut split = x.split(" ");
                let dest = split.next().unwrap().parse::<u32>().unwrap();
                let source = split.next().unwrap().parse::<u32>().unwrap();
                let range_length = split.next().unwrap().parse::<u32>().unwrap();
                (dest, source, range_length)
            })
            .collect::<Vec<(u32, u32, u32)>>();
        light_to_temperature_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let source_range = RangeInclusive::new(source, source + range_length - 1);
            let dest_range = RangeInclusive::new(dest, dest + range_length - 1);
            let mapping = SourceDestMapping {
                source_range,
                dest_range,
            };

            self.light_to_temperature_mapping.mappings.push(mapping);
        });

        let temperature_to_humidity =
            &lines[temperature_to_humidity_index + 1..humidity_to_location_index];
        let temperature_to_humidity_parsed = temperature_to_humidity
            .iter()
            .map(|x| {
                let mut split = x.split(" ");
                let dest = split.next().unwrap().parse::<u32>().unwrap();
                let source = split.next().unwrap().parse::<u32>().unwrap();
                let range_length = split.next().unwrap().parse::<u32>().unwrap();
                (dest, source, range_length)
            })
            .collect::<Vec<(u32, u32, u32)>>();
        temperature_to_humidity_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let source_range = RangeInclusive::new(source, source + range_length - 1);
            let dest_range = RangeInclusive::new(dest, dest + range_length - 1);
            let mapping = SourceDestMapping {
                source_range,
                dest_range,
            };

            self.temperature_to_humidity_mapping.mappings.push(mapping);
        });

        let humidity_to_location = &lines[humidity_to_location_index + 1..];
        let humidity_to_location_parsed = humidity_to_location
            .iter()
            .map(|x| {
                let mut split = x.split(" ");
                let dest = split.next().unwrap().parse::<u32>().unwrap();
                let source = split.next().unwrap().parse::<u32>().unwrap();
                let range_length = split.next().unwrap().parse::<u32>().unwrap();
                (dest, source, range_length)
            })
            .collect::<Vec<(u32, u32, u32)>>();
        humidity_to_location_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let source_range = RangeInclusive::new(source, source + range_length - 1);
            let dest_range = RangeInclusive::new(dest, dest + range_length - 1);
            let mapping = SourceDestMapping {
                source_range,
                dest_range,
            };

            self.humidity_to_location_mapping.mappings.push(mapping);
        });
    }

    fn part1(&mut self) -> Vec<String> {
        let mut lowest_location: u32 = 0;

        for seed in &self.seeds {
            let mut pointer = *seed;
            for mapping in &self.seed_to_soil_mapping.mappings {
                if mapping.source_range.contains(seed) {
                    let offset = seed - *mapping.source_range.start();
                    let dest = mapping.dest_range.start() + offset;
                    pointer = dest;
                }
            }

            for mapping in &self.soil_to_fertilizer_mapping.mappings {
                if mapping.source_range.contains(&pointer) {
                    let offset = pointer - *mapping.source_range.start();
                    let dest = mapping.dest_range.start() + offset;
                    pointer = dest;
                    break;
                }
            }

            for mapping in &self.fertilizer_to_water_mapping.mappings {
                if mapping.source_range.contains(&pointer) {
                    let offset = pointer - *mapping.source_range.start();
                    let dest = mapping.dest_range.start() + offset;
                    pointer = dest;
                    break;
                }
            }

            for mapping in &self.water_to_light_mapping.mappings {
                if mapping.source_range.contains(&pointer) {
                    let offset = pointer - *mapping.source_range.start();
                    let dest = mapping.dest_range.start() + offset;
                    pointer = dest;
                    break;
                }
            }

            for mapping in &self.light_to_temperature_mapping.mappings {
                if mapping.source_range.contains(&pointer) {
                    let offset = pointer - *mapping.source_range.start();
                    let dest = mapping.dest_range.start() + offset;
                    pointer = dest;
                    break;
                }
            }

            for mapping in &self.temperature_to_humidity_mapping.mappings {
                if mapping.source_range.contains(&pointer) {
                    let offset = pointer - *mapping.source_range.start();
                    let dest = mapping.dest_range.start() + offset;
                    pointer = dest;
                    break;
                }
            }

            for mapping in &self.humidity_to_location_mapping.mappings {
                if mapping.source_range.contains(&pointer) {
                    let offset = pointer - *mapping.source_range.start();
                    let dest = mapping.dest_range.start() + offset;
                    pointer = dest;
                    break;
                }
            }

            if pointer < lowest_location || lowest_location == 0 {
                lowest_location = pointer;
            }
        }

        vec![lowest_location.to_string()]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;

        vec![total.to_string()]
    }
}
