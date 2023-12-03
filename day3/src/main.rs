fn main() {
    let input = read_input();
    let parsed_input = parse_input(&input);

    println!("Part 1: {}", part1(&parsed_input));
}

fn part1(input: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for y in 0..input.len() {
        let mut numbers = Vec::new();
        let mut number_valid = false;
        for x in 0..input[y].len() {
            let char = input[y][x];
            if char.is_ascii_digit() {
                numbers.push(char);
                number_valid = number_valid || char_adjacent_to_symbol(input, x as u32, y as u32);
            } else {
                if number_valid {
                    let number_string: String = numbers.iter().collect();
                    sum += number_string.parse::<u32>().unwrap();
                }
                numbers.clear();
                number_valid = false;
            }
        }
        // Last of line edgecase
        if number_valid {
            let number_string: String = numbers.iter().collect();
            sum += number_string.parse::<u32>().unwrap();
        }
    }
    sum
}

fn char_adjacent_to_symbol(input: &Vec<Vec<char>>, x: u32, y: u32) -> bool {
    const TOP: (i32, i32) = (0, 1);
    const BOTTOM: (i32, i32) = (0, -1);
    const LEFT: (i32, i32) = (-1, 0);
    const RIGHT: (i32, i32) = (1, 0);
    const TOP_LEFT: (i32, i32) = (-1, 1);
    const TOP_RIGHT: (i32, i32) = (1, 1);
    const BOTTOM_LEFT: (i32, i32) = (-1, -1);
    const BOTTOM_RIGHT: (i32, i32) = (1, -1);

    let offsets = vec![
        TOP,
        BOTTOM,
        LEFT,
        RIGHT,
        TOP_LEFT,
        TOP_RIGHT,
        BOTTOM_LEFT,
        BOTTOM_RIGHT,
    ];

    for (delat_x, delta_y) in offsets {
        let y_coord = y as i32 + delta_y;
        let x_coord = x as i32 + delat_x;
        if y_coord < 0 || y_coord >= input.len() as i32 {
            continue;
        }
        if let Some(row) = input.get(y_coord as usize) {
            if x_coord < 0 || x_coord >= row.len() as i32 {
                continue;
            }
            if let Some(char) = row.get(x_coord as usize) {
                if is_symbol(char) {
                    return true;
                }
            }
        }
    }
    false
}

fn is_symbol(c: &char) -> bool {
    *c != '.' && c.is_ascii_punctuation()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn read_input() -> String {
    std::fs::read_to_string("input.txt").expect("Failed to read input.txt")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let parsed_input = parse_input(&input);
        assert_eq!(4361, part1(&parsed_input))
    }

    #[test]
    fn test_char_adjacent_to_symbol() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let parsed_input = parse_input(&input);
        assert!(char_adjacent_to_symbol(&parsed_input, 3, 2));
        assert!(char_adjacent_to_symbol(&parsed_input, 6, 2));
        assert!(!char_adjacent_to_symbol(&parsed_input, 0, 0));
        assert!(!char_adjacent_to_symbol(&parsed_input, 0, 100));
        assert!(char_adjacent_to_symbol(&parsed_input, 2, 9));
        assert!(char_adjacent_to_symbol(&parsed_input, 6, 9));
        assert!(!char_adjacent_to_symbol(&parsed_input, 7, 9));
    }
}
