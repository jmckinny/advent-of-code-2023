use std::{collections::HashMap, io::Write};

use maps::Almanac;

mod maps;
fn main() {
    let input = read_input();
    let data = parse_input(&input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(data: &Almanac) -> u64 {
    let mut locations = Vec::new();
    for seed in &data.seeds {
        let location = data.get_location(*seed);
        locations.push(location);
    }

    locations.into_iter().min().unwrap()
}

fn part2(data: &Almanac) -> u64 {
    let mut lowest_location = u64::MAX;
    // Get seed pairs
    let mut seeds = data.seeds.iter();
    let mut seed_ranges = Vec::new();
    if let Some(start) = seeds.next() {
        if let Some(range) = seeds.next() {
            seed_ranges.push((*start)..(*start + *range));
        }
    }

    for range in seed_ranges.into_iter() {
        println!("Range {:?}", range);
        for seed in range {
            let location = data.get_location(seed);
            lowest_location = lowest_location.min(location);
        }
    }

    lowest_location
}

fn parse_input(input: &str) -> Almanac {
    input.parse().unwrap()
}

fn read_input() -> String {
    std::fs::read_to_string("input.txt").expect("Failed to read input")
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "seeds: 79 14 55 13

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
56 93 4";

    #[test]
    fn test_part1() {
        let data = parse_input(SAMPLE_INPUT);
        assert_eq!(35, part1(&data));
    }

    #[test]
    fn test_part2() {
        let data = parse_input(SAMPLE_INPUT);
        assert_eq!(46, part2(&data));
    }

    #[test]
    fn test_parse() {
        let data = parse_input(SAMPLE_INPUT);
        assert_eq!(data.seeds[0], 79);
        assert_eq!(*data.seeds.iter().last().unwrap(), 13);

        assert_eq!(data.maps[0].get_from(), "seed");
        assert_eq!(data.maps[0].get_to(), "soil");
        assert_eq!(data.maps[0].entries[0].destination_range_start, 50);
        assert_eq!(data.maps[0].entries[0].source_range_start, 98);
        assert_eq!(data.maps[0].entries[0].range_length, 2);

        assert_eq!(data.maps[1].get_from(), "soil");
        assert_eq!(data.maps[1].get_to(), "fertilizer");
        assert_eq!(data.maps[1].entries[1].destination_range_start, 37);
        assert_eq!(data.maps[1].entries[1].source_range_start, 52);
        assert_eq!(data.maps[1].entries[1].range_length, 2);
    }

    #[test]
    fn test_map_range() {
        let data = parse_input(SAMPLE_INPUT);
        let corresponds_to1 = data
            .maps
            .first()
            .unwrap()
            .entries
            .first()
            .unwrap()
            .number_in_entry(99);
        assert_eq!(Some(51), corresponds_to1);
        let corresponds_to2 = data
            .maps
            .first()
            .unwrap()
            .entries
            .first()
            .unwrap()
            .number_in_entry(98);
        assert_eq!(Some(50), corresponds_to2);

        let corresponds_to3 = data
            .maps
            .first()
            .unwrap()
            .entries
            .iter()
            .nth(1)
            .unwrap()
            .number_in_entry(53);
        assert_eq!(Some(55), corresponds_to3);
    }
}
