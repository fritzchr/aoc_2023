#[derive(Debug, Default)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
    #[default]
    Empty,
}

impl CubeColor {
    fn from(color_as_string: &str) -> CubeColor {
        match color_as_string {
            "red" => CubeColor::Red,
            "green" => CubeColor::Green,
            "blue" => CubeColor::Blue,
            _ => CubeColor::Empty,
        }
    }
}

#[derive(Debug, Default)]
pub struct Cube {
    amount: u32,
    color: CubeColor,
}

impl Cube {
    fn new(amount: u32, color: CubeColor) -> Cube {
        Cube { amount, color }
    }
}

#[derive(Debug, Default)]
pub struct GameRecord {
    pub shown_cubes: Vec<Cube>,
}

#[derive(Debug, Default)]
pub struct Game {
    pub id: u32,
    pub record: GameRecord,
}

impl Game {
    pub fn new(id: u32, record: GameRecord) -> Game {
        Game { id, record }
    }

    pub fn is_possible(&self) -> bool {
        const MAX_RED_CUBES: u32 = 12;
        const MAX_GREEN_CUBES: u32 = 13;
        const MAX_BLUE_CUBES: u32 = 14;

        let mut is_possible = true;
        for cube in &self.record.shown_cubes {
            match cube.color {
                CubeColor::Red => {
                    if cube.amount > MAX_RED_CUBES {
                        is_possible = false;
                        break;
                    }
                }
                CubeColor::Green => {
                    if cube.amount > MAX_GREEN_CUBES {
                        is_possible = false;
                        break;
                    }
                }
                CubeColor::Blue => {
                    if cube.amount > MAX_BLUE_CUBES {
                        is_possible = false;
                        break;
                    }
                }
                _ => panic!("Unknown cube"),
            }
        }
        is_possible
    }

    pub fn get_lowest_possible_cubes(&self) -> (u32, u32, u32) {
        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;

        for cube in &self.record.shown_cubes {
            match cube.color {
                CubeColor::Red => {
                    if cube.amount > max_red {
                        max_red = cube.amount;
                    }
                }
                CubeColor::Green => {
                    if cube.amount > max_green {
                        max_green = cube.amount;
                    }
                }
                CubeColor::Blue => {
                    if cube.amount > max_blue {
                        max_blue = cube.amount;
                    }
                }
                _ => panic!("Unknown cube"),
            }
        }

        (max_red, max_green, max_blue)
    }
}

fn extract_game_id(id_string: &str) -> u32 {
    let it = id_string.split_whitespace();
    let id = it.last().map(|id| match id.parse::<u32>() {
        Ok(id) => id,
        Err(_) => panic!("Invalid id string"),
    });
    id.unwrap()
}

fn extract_game_record(record_string: &str) -> GameRecord {
    let mut game_record = GameRecord::default();

    let record_sets = record_string.split_terminator(';');
    for set in record_sets {
        let cube_str = set.split_terminator(',');
        for str in cube_str {
            let digit_color_pair = str.split_whitespace();

            let mut amount: Option<u32> = None;
            let mut color: Option<CubeColor> = None;
            for entry in digit_color_pair {
                match entry.parse::<u32>() {
                    Ok(digit) => amount = Some(digit),
                    Err(_) => color = Some(CubeColor::from(entry)),
                }
            }
            if let (Some(digit), Some(color)) = (amount, color) {
                game_record.shown_cubes.push(Cube::new(digit, color));
            }
        }
    }
    game_record
}

pub fn extract_game_information(text: &str) -> Game {
    let mut game_entry_it = text.split_terminator(':');

    let mut id = 0;
    if let Some(id_string) = game_entry_it.next() {
        id = extract_game_id(id_string);
    }

    let mut record = GameRecord::default();
    if let Some(record_string) = game_entry_it.next() {
        record = extract_game_record(record_string);
    }

    Game::new(id, record)
}

fn assignment01(input: &String) -> u32 {
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        games.push(extract_game_information(line));
    }

    let mut sum = 0;
    for game in games {
        if game.is_possible() {
            sum += game.id;
        }
    }
    sum
}

fn assignment02(input: &String) -> u32 {
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        games.push(extract_game_information(line));
    }

    let mut sum = 0;
    for game in games {
        let (max_red, max_green, max_blue) = game.get_lowest_possible_cubes();
        let pow = max_red * max_green * max_blue;
        sum += pow;
    }
    sum
}

pub fn day02() {
    let input = aoc_2023::read_input("input/day02.txt").expect("Failed to read file");

    let result_01 = assignment01(&input);
    let result_02 = assignment02(&input);

    println!("Day 02 - Assignment 01 solution: {result_01}");
    println!("Day 02 - Assignment 02 solution: {result_02}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green;
";

    #[test]
    fn assignment01_test() {
        assert_eq!(assignment01(&EXAMPLE_DATA.to_owned()), 8);
    }
    #[test]
    fn assignment02_test() {
        assert_eq!(assignment02(&EXAMPLE_DATA.to_owned()), 2286);
    }
}
