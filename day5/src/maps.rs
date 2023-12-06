use std::str::FromStr;

pub type Seed = u64;

#[derive(Debug, Clone)]
pub struct Almanac {
    pub seeds: Vec<Seed>,
    pub maps: Vec<Map>,
}

impl Almanac {
    pub fn get_location(&self, seed: Seed) -> u64 {
        let mut value = seed;
        for map in &self.maps {
            let hits = map
                .entries
                .iter()
                .filter_map(|entry| entry.number_in_entry(value))
                .collect::<Vec<u64>>();
            if let Some(new_value) = hits.first() {
                value = *new_value;
            }
        }
        value
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    from: String,
    to: String,
    pub entries: Vec<MapEntry>,
}

impl Map {
    pub fn get_from(&self) -> &str {
        &self.from
    }

    pub fn get_to(&self) -> &str {
        &self.to
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let title_line = lines.next().unwrap();
        let mut split = title_line.split("-to-");
        let from = split.next().unwrap().to_string();
        let to = split
            .last()
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .to_string();

        let entries: Vec<MapEntry> = lines
            .map(|line| {
                let numbers = line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                MapEntry::from_nums(numbers)
            })
            .collect();

        Ok(Map { from, to, entries })
    }
}

#[derive(Debug, Clone)]
pub struct MapEntry {
    pub destination_range_start: u64,
    pub source_range_start: u64,
    pub range_length: u64,
}

impl MapEntry {
    pub fn number_in_entry(&self, number: u64) -> Option<u64> {
        let source_range = self.source_range_start..self.source_range_start + self.range_length;
        if !source_range.contains(&number) {
            None
        } else {
            let index = (number - source_range.start) as usize;
            let dest_range =
                self.destination_range_start..(self.destination_range_start + self.range_length);
            Some(dest_range.into_iter().nth(index).unwrap())
        }
    }
}

impl MapEntry {
    pub fn from_nums(data: Vec<u64>) -> Self {
        MapEntry {
            destination_range_start: data[0],
            source_range_start: data[1],
            range_length: data[2],
        }
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seeds = s
            .lines()
            .next()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let maps = s
            .split("\n\n")
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Almanac { seeds, maps })
    }
}
