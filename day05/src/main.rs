use nom::{
    IResult,
    bytes::complete::tag,
    multi::{separated_list1, many1},
    character::complete::{u64, space1},
};
use itertools::Itertools;
use std::ops::Range;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 173706076);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 11611182);
}

fn part1(input: &str) -> u64 {
    let input = Input::parse(input);

    input.seeds
        .iter()
        .map(|&seed| input.seed_to_location(seed))
        .min()
        .expect("at least one seed")
}

fn part2(input: &str) -> u64 {
    let input = Input::parse(input);
    let seed_ranges = input.seeds.iter().tuples::<(&u64, &u64)>();

    #[allow(clippy::map_flatten)]
    seed_ranges
        .map(|(&start, &length)| input.seeds_to_locations(start..(start+length)))
        .flatten()
        .map(|range| range.start)
        .min()
        .expect("at least one seed")
}

fn rangemap(input: &str) -> IResult<&str, RangeMap> {
    let (input, dest_start) = u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, src_start) = u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, length) = u64(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, RangeMap {dest_start, src_start, length}))
}

fn rangemapper(input: &str) -> IResult<&str, RangeMapper> {
    let (input, mut ranges) = many1(rangemap)(input)?;
    // Sort the ranges by source values
    ranges.sort_unstable_by_key(|range| range.src_start);
    Ok((input, RangeMapper {ranges}))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(space1, u64)(input)?;
    let (input, _) = tag("\n\nseed-to-soil map:\n")(input)?;
    let (input, seed_to_soil) = rangemapper(input)?;
    let (input, _) = tag("\nsoil-to-fertilizer map:\n")(input)?;
    let (input, soil_to_fertilizer) = rangemapper(input)?;
    let (input, _) = tag("\nfertilizer-to-water map:\n")(input)?;
    let (input, fertilizer_to_water) = rangemapper(input)?;
    let (input, _) = tag("\nwater-to-light map:\n")(input)?;
    let (input, water_to_light) = rangemapper(input)?;
    let (input, _) = tag("\nlight-to-temperature map:\n")(input)?;
    let (input, light_to_temp) = rangemapper(input)?;
    let (input, _) = tag("\ntemperature-to-humidity map:\n")(input)?;
    let (input, temp_to_humid) = rangemapper(input)?;
    let (input, _) = tag("\nhumidity-to-location map:\n")(input)?;
    let (input, humid_to_location) = rangemapper(input)?;
    
    Ok((input,
        Input {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humid,
            humid_to_location
        }
    ))
}

struct Input {
    seeds: Vec<u64>,
    seed_to_soil: RangeMapper,
    soil_to_fertilizer: RangeMapper,
    fertilizer_to_water: RangeMapper,
    water_to_light: RangeMapper,
    light_to_temp: RangeMapper,
    temp_to_humid: RangeMapper,
    humid_to_location: RangeMapper
}

impl Input {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temp = self.light_to_temp.map(light);
        let humid = self.temp_to_humid.map(temp);
        self.humid_to_location.map(humid)
    }

    #[allow(clippy::map_flatten)]
    fn seeds_to_locations(&self, seeds: Range<u64>) -> Vec<Range<u64>> {
        // println!("seeds: {seeds:?}");
        let length = seeds.end - seeds.start;
        let soils = self.seed_to_soil.map_range(seeds);
        // println!("soils: {soils:?}");
        let fertilizers = soils.into_iter().map(|range| self.soil_to_fertilizer.map_range(range)).flatten().collect_vec();
        // println!("fertilizers: {fertilizers:?}");
        let waters = fertilizers.into_iter().map(|range| self.fertilizer_to_water.map_range(range)).flatten().collect_vec();
        // println!("waters: {waters:?}");
        let lights = waters.into_iter().map(|range| self.water_to_light.map_range(range)).flatten().collect_vec();
        // println!("lights: {lights:?}");
        let temps = lights.into_iter().map(|range| self.light_to_temp.map_range(range)).flatten().collect_vec();
        // println!("temps: {temps:?}");
        let humids = temps.into_iter().map(|range| self.temp_to_humid.map_range(range)).flatten().collect_vec();
        // println!("humids: {humids:?}");
        let locations = humids.into_iter().map(|range| self.humid_to_location.map_range(range)).flatten().collect_vec();
        // println!("locations: {locations:?}");

        assert_eq!(length, locations.iter().map(|r| r.end-r.start).sum());
        locations
    }

    fn parse(input: &str) -> Self {
        input_parser(input)
            .expect("unable to parse")
            .1
    }
}

struct RangeMap {
    dest_start: u64,
    src_start: u64,
    length: u64
}

impl RangeMap {
    fn map(&self, src: u64) -> Option<u64> {
        if src >= self.src_start && src < (self.src_start + self.length) {
            Some(self.dest_start + (src - self.src_start))
        } else {
            None
        }
    }
}

struct RangeMapper {
    ranges: Vec<RangeMap>
}

impl RangeMapper {
    fn map(&self, src:u64) -> u64 {
        for range in self.ranges.iter() {
            if let Some(result) = range.map(src) {
                return result;
            }
        }
        src
    }

    #[allow(clippy::map_flatten)]
    fn map_range(&self, src: Range<u64>) -> Vec<Range<u64>> {
        // Assumes that the ranges are sorted by src_start
        let mut start = src.start;
        let end = src.end;
        let mut result = vec![];

        for rm in self.ranges.iter() {
            if start >= end {
                break;
            }
            if start < rm.src_start {
                // An unmapped area before the start of this range
                let this_end = end.min(rm.src_start);
                result.push(start .. this_end);
                start = this_end;
            }
            let rm_src_end = rm.src_start + rm.length;
            if start < end && start < (rm_src_end) {
                let this_start = start.max(rm.src_start);
                let this_end = end.min(rm_src_end);
                let this_length = this_end - this_start;
                let dest_start = this_start - rm.src_start + rm.dest_start;
                result.push(dest_start .. (dest_start + this_length));
                start = this_end;
            }
        }

        // Handle an area after the last RangeMap
        if start < end {
            result.push(start .. end);
        }

        result
    }
}

#[cfg(test)]
const EXAMPLE1_STR: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

#[test]
fn test_part1_example1() {
    assert_eq!(part1(EXAMPLE1_STR), 35);
}

#[test]
fn test_part2_example1() {
    assert_eq!(part2(EXAMPLE1_STR), 46);
}