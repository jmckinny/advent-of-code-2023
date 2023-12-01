fn main() {
    let input = load_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let tokens = tokenize(line);
            let first = tokens.first().unwrap();
            let last = tokens.iter().last().unwrap();
            digits_to_num(*first, *last)
        })
        .sum()
}
#[derive(Debug, Clone, Copy)]
enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<char> for Digit {
    fn from(val: char) -> Self {
        match val {
            '1' => Digit::One,
            '2' => Digit::Two,
            '3' => Digit::Three,
            '4' => Digit::Four,
            '5' => Digit::Five,
            '6' => Digit::Six,
            '7' => Digit::Seven,
            '8' => Digit::Eight,
            '9' => Digit::Nine,
            _ => panic!("Unreachable"),
        }
    }
}

impl ToString for Digit {
    fn to_string(&self) -> String {
        match self {
            Digit::One => "1".to_string(),
            Digit::Two => "2".to_string(),
            Digit::Three => "3".to_string(),
            Digit::Four => "4".to_string(),
            Digit::Five => "5".to_string(),
            Digit::Six => "6".to_string(),
            Digit::Seven => "7".to_string(),
            Digit::Eight => "8".to_string(),
            Digit::Nine => "9".to_string(),
        }
    }
}

fn digits_to_num(a: Digit, b: Digit) -> u32 {
    let mut first = a.to_string();
    first.push_str(&b.to_string());
    first.parse().unwrap()
}

fn tokenize(str: &str) -> Vec<Digit> {
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < str.len() {
        let c = str.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            tokens.push(c.into());
            i += 1;
            continue;
        }

        let slice = &str[i..];
        if slice.starts_with("one") {
            tokens.push(Digit::One);
        } else if slice.starts_with("two") {
            tokens.push(Digit::Two);
        } else if slice.starts_with("three") {
            tokens.push(Digit::Three);
        } else if slice.starts_with("four") {
            tokens.push(Digit::Four);
        } else if slice.starts_with("five") {
            tokens.push(Digit::Five);
        } else if slice.starts_with("six") {
            tokens.push(Digit::Six);
        } else if slice.starts_with("seven") {
            tokens.push(Digit::Seven);
        } else if slice.starts_with("eight") {
            tokens.push(Digit::Eight);
        } else if slice.starts_with("nine") {
            tokens.push(Digit::Nine);
        }
        i += 1;
    }
    tokens
}

fn part1(input: &str) -> u32 {
    input.lines().map(get_number_from_line).sum()
}

fn get_number_from_line(line: &str) -> u32 {
    let first = line.chars().find(|c| c.is_ascii_digit()).unwrap_or('0');
    let last = line
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .unwrap_or('0');
    let mut num_string = String::new();
    num_string.push(first);
    num_string.push(last);
    num_string.parse().unwrap()
}

fn load_input() -> String {
    let filename = "input.txt";
    std::fs::read_to_string(filename).expect("Failed to read input.txt")
}
