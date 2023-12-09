use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Eq, PartialEq, PartialOrd, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::FiveOfAKind, Self::FiveOfAKind) => Ordering::Equal,
            (Self::FiveOfAKind, _) => Ordering::Greater,

            (Self::FourOfAKind, Self::FourOfAKind) => Ordering::Equal,
            (Self::FourOfAKind, Self::FiveOfAKind) => Ordering::Less,
            (Self::FourOfAKind, _) => Ordering::Greater,

            (Self::FullHouse, Self::FullHouse) => Ordering::Equal,
            (Self::FullHouse, Self::FiveOfAKind) | (Self::FullHouse, Self::FourOfAKind) => {
                Ordering::Less
            }
            (Self::FullHouse, _) => Ordering::Greater,

            (Self::ThreeOfAKind, Self::ThreeOfAKind) => Ordering::Equal,
            (Self::ThreeOfAKind, Self::FiveOfAKind)
            | (Self::ThreeOfAKind, Self::FourOfAKind)
            | (Self::ThreeOfAKind, Self::FullHouse) => Ordering::Less,
            (Self::ThreeOfAKind, _) => Ordering::Greater,

            (Self::TwoPair, Self::TwoPair) => Ordering::Equal,
            (Self::TwoPair, Self::OnePair) | (Self::TwoPair, Self::HighCard) => Ordering::Greater,
            (Self::TwoPair, _) => Ordering::Less,

            (Self::OnePair, Self::OnePair) => Ordering::Equal,
            (Self::OnePair, Self::HighCard) => Ordering::Greater,
            (Self::OnePair, _) => Ordering::Less,

            (Self::HighCard, Self::HighCard) => Ordering::Equal,
            (Self::HighCard, _) => Ordering::Less,
        }
    }
}

#[derive(Debug, Eq, PartialOrd)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    r#type: HandType,
}

impl Hand {
    fn new(cards: Vec<char>, bid: usize) -> Self {
        let r#type = Self::determine_type(&cards);
        Hand { cards, bid, r#type }
    }

    fn new_with_joker(cards: Vec<char>, bid: usize) -> Self {
        let r#type = Self::determine_type_with_joker(&cards);
        Hand { cards, bid, r#type }
    }

    #[inline]
    fn calculate_type(card_types: HashMap<char, usize>) -> HandType {
        match card_types.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                if card_types.values().any(|&count| count == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            2 => {
                if card_types.values().any(|&count| count == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            1 => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }

    fn determine_type(cards: &Vec<char>) -> HandType {
        let mut card_types: HashMap<char, usize> = HashMap::new();

        for c in cards {
            let count = card_types.entry(*c).or_insert(0);
            *count += 1;
        }

        Self::calculate_type(card_types)
    }

    fn determine_type_with_joker(cards: &Vec<char>) -> HandType {
        let mut card_types: HashMap<char, usize> = HashMap::new();
        let mut best_hand_type = HandType::HighCard;

        for c in cards {
            let count = card_types.entry(*c).or_insert(0);
            *count += 1;
        }

        if card_types.contains_key(&'J') {
            if *card_types.get(&'J').unwrap_or(&0) == 5 {
                // There might be a hand full of jokers
                return HandType::FiveOfAKind;
            }

            for (c, d) in &card_types {
                if *c != 'J' {
                    let mut cards: HashMap<char, usize> = card_types.clone();
                    let joker_cnt = cards.remove(&'J').unwrap_or(0);

                    if let Some(card) = cards.get_mut(c) {
                        *card = d + joker_cnt;
                    }

                    let current_hand_type = Self::calculate_type(cards);
                    if current_hand_type.cmp(&best_hand_type) == Ordering::Greater {
                        best_hand_type = current_hand_type;
                    }
                }
            }
        } else {
            best_hand_type = Self::determine_type(cards);
        }
        best_hand_type
    }

    /// I'm using a custom comparison function instead of implementing Ord.
    /// Mainly to pass in a custom set of ordering rules depending on the assignment.
    fn cmp(&self, other: &Self, ordering: &str) -> Ordering {
        match self.r#type.cmp(&other.r#type) {
            Ordering::Equal => {
                // If the hand has the same type, the hand
                // with the stronger starting card is superior.
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    match ordering.find(*b).cmp(&ordering.find(*a)) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                // In this case, all characters are equal
                Ordering::Equal
            }
            other => other,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn extract_input_01(input: &str) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];

    for line in input.lines() {
        let mut entry = line.split_whitespace();

        let cards: Vec<char> = entry.next().unwrap().chars().collect();
        let bid = entry
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Failed to parse value");

        hands.push(Hand::new(cards, bid));
    }

    hands
}

fn extract_input_02(input: &str) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];

    for line in input.lines() {
        let mut entry = line.split_whitespace();

        let cards: Vec<char> = entry.next().unwrap().chars().collect();
        let bid = entry
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Failed to parse value");

        hands.push(Hand::new_with_joker(cards, bid));
    }

    hands
}

fn assignment01(input: &str) -> usize {
    const ORDERING: &str = "AKQJT98765432";
    let mut hands = extract_input_01(input);

    hands.sort_by(|a, b| a.cmp(&b, ORDERING));

    let mut score = 1;
    let mut total_winnings = 0;

    for hand in hands {
        total_winnings = total_winnings + (hand.bid * score);
        score += 1;
    }
    total_winnings
}

fn assignment02(input: &str) -> usize {
    const ORDERING: &str = "AKQT98765432J";
    let mut hands = extract_input_02(input);

    hands.sort_by(|a, b| a.cmp(&b, ORDERING));

    let mut score = 1;
    let mut total_winnings = 0;

    for hand in hands {
        total_winnings = total_winnings + (hand.bid * score);
        score += 1;
    }
    total_winnings
}

pub fn day07() {
    let input = aoc_2023::read_input("input/day07.txt").expect("Failed to read file");

    let result_01 = assignment01(&input);
    let result_02 = assignment02(&input);

    println!("Day 07 - Assignment 01 solution: {result_01}");
    println!("Day 07 - Assignment 02 solution: {result_02}");
    println!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn assignment01_test() {
        assert_eq!(assignment01(&EXAMPLE_DATA.to_owned()), 6440);
    }

    #[test]
    fn assignment02_test() {
        assert_eq!(assignment02(&EXAMPLE_DATA.to_owned()), 5905);
    }
}
