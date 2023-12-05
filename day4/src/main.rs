mod card;
use card::Card;
use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = read_input();
    let cards = parse_cards(&input);
    println!("Part 1: {}", part1(&cards));
    println!("Part 2: {}", part2(&cards));
}

fn part1(cards: &[Card]) -> u32 {
    cards.iter().map(|card| card.get_point_value()).sum()
}

fn part2(cards: &Vec<Card>) -> u32 {
    let mut copies: HashMap<u32, u32> = HashMap::new();
    for card in cards {
        let point_value = card.get_matches();
        let card_number = card.get_card_num();
        copies.entry(card_number).or_insert(1);

        for _ in 0..(*copies.get(&card_number).unwrap()) {
            for i in (card_number + 1)..=(card_number + point_value) {
                copies.entry(i).or_insert(1);
                let copy_count = copies.get_mut(&i).unwrap();
                *copy_count += 1;
            }
        }
    }
    copies.values().sum()
}

fn read_input() -> String {
    std::fs::read_to_string("input.txt").expect("Failed to read input.txt")
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| Card::from_str(line).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards = parse_cards(&input);
        assert_eq!(13, part1(&cards));
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards = parse_cards(&input);
        assert_eq!(30, part2(&cards));
    }
}
