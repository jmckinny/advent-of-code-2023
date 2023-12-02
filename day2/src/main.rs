use std::str::FromStr;

use game::{Color, Game};

mod game;
fn main() {
    let input = read_input();
    let games = load_games(&input);
    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));
}

fn part1(games: &[Game]) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    games
        .iter()
        .filter(|game| {
            let game_red_max = game.get_max_color_count(Color::Red);
            let game_green_max = game.get_max_color_count(Color::Green);
            let game_blue_max = game.get_max_color_count(Color::Blue);
            game_red_max <= MAX_RED && game_green_max <= MAX_GREEN && game_blue_max <= MAX_BLUE
        })
        .map(|game| game.get_id())
        .sum()
}

fn part2(game: &[Game]) -> u32 {
    game.iter()
        .map(|game| {
            let red_count = game.get_max_color_count(Color::Red);
            let green_count = game.get_max_color_count(Color::Green);
            let blue_count = game.get_max_color_count(Color::Blue);
            red_count * green_count * blue_count
        })
        .sum()
}

fn load_games(text: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for line in text.lines() {
        games.push(Game::from_str(line).unwrap());
    }
    games
}

fn read_input() -> String {
    std::fs::read_to_string("input.txt").expect("Failed to read input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let part1_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = load_games(part1_input);
        assert_eq!(8, part1(&games));
    }

    #[test]
    fn test_part2() {
        let part2_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = load_games(part2_input);
        assert_eq!(2286, part2(&games));
    }
}
