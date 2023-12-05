use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Card {
    id: usize,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

impl Card {
    fn new(id: usize, winning_numbers: Vec<u32>, drawn_numbers: Vec<u32>) -> Card {
        Card {
            id,
            winning_numbers,
            drawn_numbers,
        }
    }

    fn get_card_score(&self) -> u32 {
        let mut score = 0;

        for number in &self.winning_numbers {
            if self.drawn_numbers.contains(&number) {
                if score == 0 {
                    score = 1;
                } else if score == 1 {
                    score += score;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }

    fn get_amount_of_winning_numbers(&self) -> usize {
        let mut amount = 0;

        for number in &self.winning_numbers {
            if self.drawn_numbers.contains(&number) {
                amount += 1;
            }
        }
        amount
    }
}

fn parse_input(text: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    let mut card_id: usize = 0;

    for line in text.lines() {
        let start_pos = line.find(':').expect("Invalid input");
        let numbers = &line[start_pos + 1..];
        let mut number_parts = numbers.split_terminator('|');

        let left_part = number_parts.next().unwrap().trim();
        let right_part = number_parts.next().unwrap().trim();

        let winning_numbers: Vec<u32> = left_part
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Failed to parse number"))
            .collect();
        let drawn_numbers: Vec<u32> = right_part
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Failed to parse number"))
            .collect();

        cards.push(Card::new(card_id, winning_numbers, drawn_numbers));
        card_id += 1;
    }

    cards
}

fn calculate_total_cards(cards: Vec<Card>) -> HashMap<usize, usize> {
    let mut initial_cards: HashMap<usize, usize> =
        cards.clone().into_iter().map(|obj| (obj.id, 1)).collect();

    for card in &cards {
        let winning_amount = card.get_amount_of_winning_numbers();
        if winning_amount > 0 {
            if let Some(current_card_amount) = initial_cards.get(&card.id) {
                for _ in 0..current_card_amount.to_owned() {
                    for i in card.id + 1..card.id + 1 + winning_amount {
                        if let Some(entry) = initial_cards.get_mut(&i) {
                            *entry += 1;
                        }
                    }
                }
            }
        }
    }
    initial_cards
}

fn assignment01(input: &str) -> u32 {
    let cards = parse_input(input);

    let mut total_card_score = 0;
    for card in cards {
        total_card_score += card.get_card_score();
    }
    total_card_score
}

fn assignment02(input: &str) -> u32 {
    let cards = parse_input(input);

    let result = calculate_total_cards(cards);
    let mut sum = 0;
    for (_, amount) in result {
        sum += amount;
    }
    sum as u32
}

pub fn day04() {
    let input = aoc_2023::read_input("input/day04.txt").expect("Failed to read file");

    let result_01 = assignment01(&input);
    let result_02 = assignment02(&input);

    println!("Day 04 - Assignment 01 solution: {result_01}");
    println!("Day 04 - Assignment 02 solution: {result_02}");
    println!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn assignment01_test() {
        assert_eq!(assignment01(&EXAMPLE_DATA.to_owned()), 13);
    }

    #[test]
    fn assignment02_test() {
        assert_eq!(assignment02(&EXAMPLE_DATA.to_owned()), 30);
    }
}
