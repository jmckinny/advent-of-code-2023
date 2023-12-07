fn main() {
    let input = read_input();
    let races = parse_input(&input);

    println!("Part 1: {}", part1(&races));
    println!("Part 2: {}", part2(&races));
}

fn part1(races: &[Race]) -> u64 {
    races.iter().map(|race| race.ways_winnable()).product()
}

fn part2(races: &[Race]) -> u64 {
    let big_race_time: u64 = races
        .iter()
        .flat_map(|r| r.time.to_string().chars().collect::<Vec<char>>())
        .collect::<String>()
        .parse()
        .unwrap();

    let big_race_distance: u64 = races
        .iter()
        .flat_map(|r| r.distance.to_string().chars().collect::<Vec<char>>())
        .collect::<String>()
        .parse()
        .unwrap();

    let big_race = Race {
        time: big_race_time,
        distance: big_race_distance,
    };
    big_race.ways_winnable()
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let distances: Vec<u64> = lines
        .last()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn read_input() -> String {
    std::fs::read_to_string("input.txt").expect("Failed to open input")
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn compute_race_length(&self, time_held: u64) -> u64 {
        if time_held >= self.time {
            return 0;
        }

        let time_moving = self.time - time_held;
        let speed = time_held;
        let distance = time_moving * speed;
        distance
    }

    fn does_time_beat_record(&self, time_held: u64) -> bool {
        self.compute_race_length(time_held) > self.distance
    }

    fn ways_winnable(&self) -> u64 {
        (0..=self.time)
            .filter(|time| self.does_time_beat_record(*time))
            .count() as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    #[test]
    fn test_part1() {
        let races = parse_input(SAMPLE_INPUT);
        assert_eq!(288, part1(&races));
    }
    #[test]
    fn test_part2() {
        let races = parse_input(SAMPLE_INPUT);
        assert_eq!(71503, part2(&races));
    }
    #[test]
    fn test_race_calc() {
        let races = parse_input(SAMPLE_INPUT);
        let first_race = races.first().unwrap();

        assert_eq!(6, first_race.compute_race_length(1));
        assert_eq!(10, first_race.compute_race_length(2));
        assert_eq!(12, first_race.compute_race_length(3));
        assert_eq!(12, first_race.compute_race_length(4));
        assert_eq!(0, first_race.compute_race_length(7));
    }
}
