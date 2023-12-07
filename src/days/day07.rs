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

    fn determine_type(cards: &Vec<char>) -> HandType {
        let mut card_type_count: HashMap<char, usize> = HashMap::new();

        for c in cards {
            let count = card_type_count.entry(*c).or_insert(0);
            *count += 1;
        }

        let mut unique_types: Vec<_> = card_type_count.keys().collect();
        unique_types.sort_by(|a, b| card_type_count[b].cmp(&card_type_count[a]));

        match card_type_count.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                if card_type_count.values().any(|&count| count == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            2 => {
                if card_type_count.values().any(|&count| count == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            1 => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        const ORDERING: &str = "AKQJT98765432";

        match self.r#type.cmp(&other.r#type) {
            Ordering::Equal => {
                // If the hand has the same type, the hand
                // with the stronger starting card is superior.
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    match ORDERING.find(*b).cmp(&ORDERING.find(*a)) {
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

fn extract_input(input: &str) -> Vec<Hand> {
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

fn assignment01(input: &str) -> usize {
    let mut hands = extract_input(input);
    hands.sort_by(|a, b| a.cmp(&b));

    let mut score = 1;
    let mut total_winnings = 0;

    for hand in hands {
        total_winnings = total_winnings + (hand.bid * score);
        score += 1;
    }
    total_winnings
}

fn assignment02(input: &str) -> u32 {
    0
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

    // #[test]
    // fn assignment02_test() {
    //     assert_eq!(assignment02(&EXAMPLE_DATA.to_owned()), 71503);
    // }
}
