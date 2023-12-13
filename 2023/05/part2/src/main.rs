use std::cmp;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../input");

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    seed_ranges: Vec<(usize, usize)>,
    seeds_to_soil: AlmanacMap,
    soil_to_fertilizer: AlmanacMap,
    fertilizer_to_water: AlmanacMap,
    water_to_light: AlmanacMap,
    light_to_temperature: AlmanacMap,
    temperature_to_humidity: AlmanacMap,
    humidity_to_location: AlmanacMap,
}

#[derive(Debug, PartialEq, Eq)]
struct AlmanacMap {
    instances: Vec<AlmanacMapInstance>,
}

#[derive(Debug, PartialEq, Eq)]
struct AlmanacMapInstance {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl Almanac {
    fn get_seed_location(&self, start: usize, range: usize) -> Vec<(usize, usize)> {
        let rngs: Vec<(usize, usize)> = vec![(start, start + range)];

        let soil = self.seeds_to_soil.get_destination(&rngs);
        let fertilizer = self.soil_to_fertilizer.get_destination(&soil);
        let water = self.fertilizer_to_water.get_destination(&fertilizer);
        let light = self.water_to_light.get_destination(&water);
        let temperature = self.light_to_temperature.get_destination(&light);
        let humidity = self.temperature_to_humidity.get_destination(&temperature);
        let location = self.humidity_to_location.get_destination(&humidity);

        // println!("{seed} {soil} {fertilizer} {water} {light} {temperature} {humidity} {location}");

        return location;
    }
}

impl AlmanacMap {
    fn get_destination(&self, rngs: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut next = rngs.clone();
        let mut mapped: Vec<(usize, usize)> = Vec::new();

        for instance in self.instances.iter() {
            let curr = next.clone();
            next.clear();

            let i_start = instance.source_range_start;
            let i_end = instance.source_range_start + instance.range_length;

            for (r_start, r_end) in curr.iter() {
                let before: (usize, usize) = (*r_start, cmp::min(*r_end, i_start));
                let intersect: (usize, usize) =
                    (cmp::max(*r_start, i_start), cmp::min(*r_end, i_end));
                let after: (usize, usize) = (cmp::max(*r_start, i_end), *r_end);

                // if there's anything leftover before, process in next iteration
                if before.0 < before.1 {
                    next.push(before);
                }

                // if anything intersects, map that shiz
                if intersect.0 < intersect.1 {
                    let offset = intersect.0 - instance.source_range_start;
                    let range_diff = intersect.1 - intersect.0;
                    let result = (
                        instance.destination_range_start + offset,
                        instance.destination_range_start + offset + range_diff,
                    );
                    mapped.push(result);
                }

                // if there's anything leftover after, process it next iteration
                if after.0 < after.1 {
                    next.push(after);
                }
            }
        }

        mapped.append(&mut next);

        return mapped;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseAlmanacError;

impl FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = s.split("\n\n").collect::<Vec<&str>>();

        let seeds = groups[0]
            .split_once(':')
            .ok_or(ParseAlmanacError)?
            .1
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().map_err(|_| ParseAlmanacError))
            .collect::<Result<Vec<usize>, _>>()?;

        let seed_ranges = seeds.chunks(2).map(|c| (c[0], c[1])).collect();

        let seeds_to_soil = groups[1].parse::<AlmanacMap>()?;
        let soil_to_fertilizer = groups[2].parse::<AlmanacMap>()?;
        let fertilizer_to_water = groups[3].parse::<AlmanacMap>()?;
        let water_to_light = groups[4].parse::<AlmanacMap>()?;
        let light_to_temperature = groups[5].parse::<AlmanacMap>()?;
        let temperature_to_humidity = groups[6].parse::<AlmanacMap>()?;
        let humidity_to_location = groups[7].parse::<AlmanacMap>()?;

        let almanac = Almanac {
            seed_ranges,
            seeds_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        };

        return Ok(almanac);
    }
}

impl FromStr for AlmanacMap {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instances = s
            .lines()
            .skip(1)
            .map(|l| l.parse())
            .collect::<Result<Vec<AlmanacMapInstance>, _>>()?;

        let map = AlmanacMap { instances };

        return Ok(map);
    }
}

impl FromStr for AlmanacMapInstance {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split(' ')
            .map(|n| n.parse().map_err(|_| ParseAlmanacError))
            .collect::<Result<Vec<usize>, _>>()?;

        let instance = AlmanacMapInstance {
            destination_range_start: nums[0],
            source_range_start: nums[1],
            range_length: nums[2],
        };

        return Ok(instance);
    }
}

fn main() {
    let almanac = INPUT.parse::<Almanac>().unwrap();

    let rngs: Vec<_> = almanac
        .seed_ranges
        .iter()
        .map(|(start, range)| almanac.get_seed_location(*start, *range))
        .map(|rngs| rngs.iter().map(|(start, _)| *start).min().unwrap())
        .collect();

    let lowest_location = rngs.iter().min().unwrap();

    dbg!(lowest_location);
}
