use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Card {
    card_num: u32,
    numbers: Vec<u32>,
    winners: Vec<u32>,
}

impl Card {
    pub fn get_card_num(&self) -> u32 {
        self.card_num
    }

    pub fn get_point_value(&self) -> u32 {
        self.numbers.iter().fold(0, |acc, x| {
            if self.winners.contains(x) {
                if acc == 0 {
                    1
                } else {
                    acc * 2
                }
            } else {
                acc
            }
        })
    }

    pub fn get_matches(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winners.contains(n))
            .count() as u32
    }
}

impl FromStr for Card {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colon_iter = s.split(':');
        let card_num = colon_iter
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()?;
        let numbers = colon_iter
            .next()
            .unwrap()
            .split('|')
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let winners = s
            .split('|')
            .last()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Card {
            card_num,
            numbers,
            winners,
        })
    }
}
