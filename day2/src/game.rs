use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    id: u32,
    handfuls: Vec<Handful>,
}

impl Game {
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_max_color_count(&self, color: Color) -> u32 {
        self.handfuls
            .iter()
            .map(|handful| match color {
                Color::Red => handful.red,
                Color::Blue => handful.blue,
                Color::Green => handful.green,
            })
            .max()
            .unwrap_or(0) // Sometimes there are none of a color
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut handfuls = Vec::new();

        let mut colon_split = s.split(':');
        let game_id_section = colon_split.next().unwrap();
        let handfuls_section = colon_split.last().unwrap();
        // ID section
        let id: u32 = game_id_section
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        // Handfuls
        for handful_str in handfuls_section.split(';') {
            for cubes_str in handful_str.split(',') {
                let mut handful = Handful::default();
                let mut whitespace = cubes_str.split_whitespace();
                let count: u32 = whitespace.next().unwrap().parse().unwrap();
                let color = whitespace.last().unwrap();
                match color {
                    "red" => handful.red += count,
                    "blue" => handful.blue += count,
                    "green" => handful.green += count,
                    _ => return Err(String::from("Invalid Color")),
                };
                handfuls.push(handful)
            }
        }

        Ok(Game { id, handfuls })
    }
}

#[derive(Debug, Default)]
pub struct Handful {
    red: u32,
    green: u32,
    blue: u32,
}

pub enum Color {
    Red,
    Blue,
    Green,
}
