#[derive(Clone, Copy, Debug, PartialEq)]
struct Cell {
    value: char,
    row: usize,
    col: usize,
}

#[derive(Clone, Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn build_from_input(input: &str) -> Grid {
        let cells: Vec<Vec<Cell>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, value)| Cell { value, row, col })
                    .collect()
            })
            .collect();

        Grid { cells }
    }

    fn get_row_size(&self) -> usize {
        self.cells.len()
    }

    fn get_col_size(&self) -> usize {
        if self.cells.len() > 0 {
            self.cells[0].len()
        } else {
            0
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<Cell> {
        self.cells.get(row)?.get(col).cloned()
    }

    fn get_adjacent_cells(&self, row: usize, col: usize) -> Vec<Cell> {
        let mut adjacent_cells: Vec<Cell> = Vec::new();

        for i in (row.saturating_sub(1))..=(row + 1).min(self.get_row_size() - 1) {
            for j in (col.saturating_sub(1))..=(col + 1).min(self.get_row_size() - 1) {
                if i != row || j != col {
                    if let Some(cell) = self.get(i, j) {
                        adjacent_cells.push(cell);
                    }
                }
            }
        }

        adjacent_cells
    }

    fn is_adjacent_to_symbol(&self, row: usize, col: usize) -> bool {
        let mut is_adjacent_to_symbol = false;
        if let Some(cell) = self.get(row, col) {
            if cell.value.is_digit(10) {
                let adjacent_cells = self.get_adjacent_cells(row, col);
                for cell in adjacent_cells {
                    if cell.value != '.' && !cell.value.is_digit(10) {
                        is_adjacent_to_symbol = true;
                    }
                }
            }
        }
        is_adjacent_to_symbol
    }

    fn is_adjacent_to_number(&self, row: usize, col: usize, symbol: char) -> bool {
        let mut is_adjacent_to_number = 0;
        if let Some(cell) = self.get(row, col) {
            if cell.value == symbol {
                let adjacent_cells = self.get_adjacent_cells(row, col);
                for cell in adjacent_cells {
                    if cell.value.is_digit(10) {
                        is_adjacent_to_number += 1;
                    }
                }
            }
        }

        is_adjacent_to_number >= 2
    }

    fn get_adjacent_numbers(&self, row: usize, col: usize) -> Vec<u32> {
        let mut part_number_cells: Vec<Vec<Cell>> = Vec::new();
        if let Some(_) = self.get(row, col) {
            let adjacent_cells = self.get_adjacent_cells(row, col);
            for cell in adjacent_cells {
                if cell.value.is_digit(10) {
                    let part_number = self.get_part_number_cells(cell.row, cell.col);
                    let mut already_considered = false;
                    for cell in &part_number_cells {
                        if cell.contains(&part_number[0]) {
                            already_considered = true;
                        }
                    }
                    if !already_considered {
                        part_number_cells.push(part_number);
                    }
                }
            }
        }
        let mut part_numbers: Vec<u32> = Vec::new();
        for part_number in part_number_cells {
            let mut part_number_string = String::new();
            for part in part_number {
                part_number_string.push(part.value);
            }
            if let Ok(number) = part_number_string.parse::<u32>() {
                part_numbers.push(number);
            } else {
                panic!("Value should be an integer");
            }
        }

        part_numbers
    }

    fn find_start_index(&self, row: usize, col: usize) -> usize {
        let mut current_col = col;

        while current_col > 0 {
            current_col -= 1;

            if let Some(cell) = self.get(row, current_col) {
                if !cell.value.is_digit(10) {
                    current_col += 1;
                    break;
                }
            }
        }
        current_col
    }

    fn find_end_index(&self, row: usize, col: usize) -> usize {
        let mut current_col = col;

        while current_col < self.get_col_size() {
            current_col += 1;

            if let Some(cell) = self.get(row, current_col) {
                if !cell.value.is_digit(10) {
                    current_col -= 1;
                    break;
                }
            }
        }
        current_col
    }

    fn get_part_number_cells(&self, row: usize, col: usize) -> Vec<Cell> {
        let mut current_col = self.find_start_index(row, col);
        let end_col = self.find_end_index(row, col);

        let mut part_number_cells: Vec<Cell> = Vec::new();

        if let Some(cell) = self.get(row, current_col) {
            part_number_cells.push(cell);

            while current_col < end_col {
                current_col += 1;

                if let Some(cell) = self.get(row, current_col) {
                    if cell.value.is_digit(10) {
                        part_number_cells.push(cell);
                    } else {
                        break;
                    }
                }
            }
        }

        part_number_cells
    }

    fn get_part_numbers(&self) -> Vec<u32> {
        let mut part_numbers_cells: Vec<Vec<Cell>> = Vec::new();

        for row in 0..self.get_row_size() {
            for col in 0..self.get_col_size() {
                if let Some(cell) = self.get(row, col) {
                    if self.is_adjacent_to_symbol(cell.row, cell.col) {
                        let part_number_cells = self.get_part_number_cells(cell.row, cell.col);
                        let mut already_considered = false;
                        for cell in &part_numbers_cells {
                            // if the first index is already part of the collection
                            // this number was already considered
                            if cell.contains(&part_number_cells[0]) {
                                already_considered = true;
                            }
                        }
                        if !already_considered {
                            part_numbers_cells.push(part_number_cells);
                        }
                    }
                }
            }
        }

        let mut part_numbers: Vec<u32> = Vec::new();
        for part_number_cells in part_numbers_cells {
            let mut part_number_string = String::new();
            for cell in part_number_cells {
                part_number_string.push(cell.value);
            }
            if let Ok(number) = part_number_string.parse::<u32>() {
                part_numbers.push(number);
            } else {
                panic!("Value should be an integer");
            }
        }
        part_numbers
    }

    fn get_gear_ratios(&self) -> Vec<u32> {
        let mut gear_ratios: Vec<u32> = Vec::new();

        for row in 0..self.get_row_size() {
            for col in 0..self.get_col_size() {
                if let Some(cell) = self.get(row, col) {
                    if self.is_adjacent_to_number(cell.row, cell.col, '*') {
                        let adjacent_numbers = self.get_adjacent_numbers(cell.row, cell.col);
                        let mut gear_ratio = 0;

                        // it is only a valid gear if there are 2 numbers
                        if adjacent_numbers.len() >= 2 {
                            for entry in adjacent_numbers {
                                if gear_ratio == 0 {
                                    gear_ratio = entry;
                                } else {
                                    gear_ratio *= entry;
                                }
                            }
                        }
                        gear_ratios.push(gear_ratio);
                    }
                }
            }
        }
        gear_ratios
    }
}

fn assignment01(input: &str) -> u32 {
    let grid = Grid::build_from_input(&input);
    let part_numbers = grid.get_part_numbers();

    let mut sum = 0;
    for part in part_numbers {
        sum += part;
    }

    sum
}

fn assignment02(input: &str) -> u32 {
    let grid = Grid::build_from_input(&input);
    let gear_ratios = grid.get_gear_ratios();

    let mut sum = 0;
    for ratio in gear_ratios {
        sum += ratio;
    }

    sum
}

pub fn day03() {
    let input = aoc_2023::read_input("input/day03.txt").expect("Failed to read file");

    let result_01 = assignment01(&input);
    let result_02 = assignment02(&input);

    println!("Day 03 - Assignment 01 solution: {result_01}");
    println!("Day 03 - Assignment 02 solution: {result_02}");
    println!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        ";

    #[test]
    fn assignment01_test() {
        assert_eq!(assignment01(&EXAMPLE_DATA.to_owned()), 4361);
    }

    #[test]
    fn assignment02_test() {
        assert_eq!(assignment02(&EXAMPLE_DATA.to_owned()), 467835);
    }
}
