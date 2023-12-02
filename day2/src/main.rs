use std::str::FromStr;

use game::Game;

mod game;
fn main() {
    let input = read_input();
    let games = load_games(&input);
    println!("Part 1: {}", part1(&games));
}

fn part1(games: &[Game]) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    games
        .iter()
        .filter(|game| {
            let game_red_max = game.get_max_color_count("red");
            let game_green_max = game.get_max_color_count("green");
            let game_blue_max = game.get_max_color_count("blue");
            game_red_max <= MAX_RED && game_green_max <= MAX_GREEN && game_blue_max <= MAX_BLUE
        })
        .map(|game| game.get_id())
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
