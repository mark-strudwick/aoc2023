use std::collections::HashMap;

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

#[derive(Debug, Default)]
struct SeedToSoil {
    mappings: HashMap<u32, u32>,
}

#[derive(Debug, Default)]
struct SoilToFertilizer {
    mappings: HashMap<u32, u32>,
}

#[derive(Debug, Default)]
struct FertilizerToWater {
    mappings: HashMap<u32, u32>,
}

#[derive(Debug, Default)]
struct WaterToLight {
    mappings: HashMap<u32, u32>,
}

#[derive(Debug, Default)]
struct LightToTemperature {
    mappings: HashMap<u32, u32>,
}

#[derive(Debug, Default)]
struct TemperatureToHumidity {
    mappings: HashMap<u32, u32>,
}

#[derive(Debug, Default)]
struct HumidityToLocation {
    mappings: HashMap<u32, u32>,
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
        let mut seed_to_soil_mapped = SeedToSoil {
            mappings: HashMap::new(),
        };
        seed_to_soil_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let dest_range = dest..dest + range_length;
            let source_range = source..source + range_length;

            dest_range.zip(source_range).for_each(|(dest, source)| {
                seed_to_soil_mapped.mappings.insert(source, dest);
            });
        });
        for i in 0..100 {
            if !seed_to_soil_mapped.mappings.contains_key(&i) {
                seed_to_soil_mapped.mappings.insert(i, i);
            }
        }

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
        let mut soil_to_fertilizer_mapped = SoilToFertilizer {
            mappings: HashMap::new(),
        };
        soil_to_fertilizer_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let dest_range = dest..dest + range_length;
            let source_range = source..source + range_length;

            dest_range.zip(source_range).for_each(|(dest, source)| {
                soil_to_fertilizer_mapped.mappings.insert(source, dest);
            });
        });
        for i in 0..100 {
            if !soil_to_fertilizer_mapped.mappings.contains_key(&i) {
                soil_to_fertilizer_mapped.mappings.insert(i, i);
            }
        }

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
        let mut fertilizer_to_water_mapped = FertilizerToWater {
            mappings: HashMap::new(),
        };
        fertilizer_to_water_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let dest_range = dest..dest + range_length;
            let source_range = source..source + range_length;

            dest_range.zip(source_range).for_each(|(dest, source)| {
                fertilizer_to_water_mapped.mappings.insert(source, dest);
            });
        });
        for i in 0..100 {
            if !fertilizer_to_water_mapped.mappings.contains_key(&i) {
                fertilizer_to_water_mapped.mappings.insert(i, i);
            }
        }

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
        let mut water_to_light_mapped = WaterToLight {
            mappings: HashMap::new(),
        };
        water_to_light_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let dest_range = dest..dest + range_length;
            let source_range = source..source + range_length;

            dest_range.zip(source_range).for_each(|(dest, source)| {
                water_to_light_mapped.mappings.insert(source, dest);
            });
        });
        for i in 0..100 {
            if !water_to_light_mapped.mappings.contains_key(&i) {
                water_to_light_mapped.mappings.insert(i, i);
            }
        }

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
        let mut light_to_temperature_mapped = LightToTemperature {
            mappings: HashMap::new(),
        };
        light_to_temperature_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let dest_range = dest..dest + range_length;
            let source_range = source..source + range_length;

            dest_range.zip(source_range).for_each(|(dest, source)| {
                light_to_temperature_mapped.mappings.insert(source, dest);
            });
        });
        for i in 0..100 {
            if !light_to_temperature_mapped.mappings.contains_key(&i) {
                light_to_temperature_mapped.mappings.insert(i, i);
            }
        }

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
        let mut temperature_to_humidity_mapped = TemperatureToHumidity {
            mappings: HashMap::new(),
        };
        temperature_to_humidity_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let dest_range = dest..dest + range_length;
            let source_range = source..source + range_length;

            dest_range.zip(source_range).for_each(|(dest, source)| {
                temperature_to_humidity_mapped.mappings.insert(source, dest);
            });
        });
        for i in 0..100 {
            if !temperature_to_humidity_mapped.mappings.contains_key(&i) {
                temperature_to_humidity_mapped.mappings.insert(i, i);
            }
        }

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
        let mut humidity_to_location_mapped = HumidityToLocation {
            mappings: HashMap::new(),
        };
        humidity_to_location_parsed.iter().for_each(|x| {
            let dest = x.0;
            let source = x.1;
            let range_length = x.2;

            let dest_range = dest..dest + range_length;
            let source_range = source..source + range_length;

            dest_range.zip(source_range).for_each(|(dest, source)| {
                humidity_to_location_mapped.mappings.insert(source, dest);
            });

            for i in 0..100 {
                if !humidity_to_location_mapped.mappings.contains_key(&i) {
                    humidity_to_location_mapped.mappings.insert(i, i);
                }
            }
        });

        self.seed_to_soil_mapping = seed_to_soil_mapped;
        self.soil_to_fertilizer_mapping = soil_to_fertilizer_mapped;
        self.fertilizer_to_water_mapping = fertilizer_to_water_mapped;
        self.water_to_light_mapping = water_to_light_mapped;
        self.light_to_temperature_mapping = light_to_temperature_mapped;
        self.temperature_to_humidity_mapping = temperature_to_humidity_mapped;
        self.humidity_to_location_mapping = humidity_to_location_mapped;
    }

    fn part1(&mut self) -> Vec<String> {
        let mut lowest_location = 4294967295;

        for seed in &self.seeds {
            let soil = self.seed_to_soil_mapping.mappings.get(seed).unwrap();
            let fertilizer = self.soil_to_fertilizer_mapping.mappings.get(soil).unwrap();
            let water = self
                .fertilizer_to_water_mapping
                .mappings
                .get(fertilizer)
                .unwrap();
            let light = self.water_to_light_mapping.mappings.get(water).unwrap();
            let temperature = self
                .light_to_temperature_mapping
                .mappings
                .get(light)
                .unwrap();
            let humidity = self
                .temperature_to_humidity_mapping
                .mappings
                .get(temperature)
                .unwrap();
            let location = self
                .humidity_to_location_mapping
                .mappings
                .get(humidity)
                .unwrap();

            if location < &lowest_location {
                lowest_location = *location;
            }
        }

        vec![lowest_location.to_string()]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;

        vec![total.to_string()]
    }
}
