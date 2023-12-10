use std::str::FromStr;

const INPUT: &'static str = include_str!("../input");

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<usize>,
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
    fn get_seed_location(&self, seed: usize) -> usize {
        let soil = self.seeds_to_soil.get_destination(seed);
        let fertilizer = self.soil_to_fertilizer.get_destination(soil);
        let water = self.fertilizer_to_water.get_destination(fertilizer);
        let light = self.water_to_light.get_destination(water);
        let temperature = self.light_to_temperature.get_destination(light);
        let humidity = self.temperature_to_humidity.get_destination(temperature);
        let location = self.humidity_to_location.get_destination(humidity);

        // println!("{seed} {soil} {fertilizer} {water} {light} {temperature} {humidity} {location}");

        return location;
    }
}

impl AlmanacMap {
    fn get_destination(&self, source: usize) -> usize {
        for instance in self.instances.iter() {
            let source_start = instance.source_range_start;
            let source_end = source_start + instance.range_length;

            if source < source_start || source > source_end {
                continue;
            }

            let offset = source - source_start;
            let destination = instance.destination_range_start + offset;

            return destination;
        }

        // didn't find a mapping, so it will be the same value
        return source;
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

        let seeds_to_soil = groups[1].parse::<AlmanacMap>()?;
        let soil_to_fertilizer = groups[2].parse::<AlmanacMap>()?;
        let fertilizer_to_water = groups[3].parse::<AlmanacMap>()?;
        let water_to_light = groups[4].parse::<AlmanacMap>()?;
        let light_to_temperature = groups[5].parse::<AlmanacMap>()?;
        let temperature_to_humidity = groups[6].parse::<AlmanacMap>()?;
        let humidity_to_location = groups[7].parse::<AlmanacMap>()?;

        let almanac = Almanac {
            seeds,
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

    let lowest_location = almanac
        .seeds
        .iter()
        .map(|s| almanac.get_seed_location(*s))
        .min()
        .unwrap();

    println!("{lowest_location}");
}
