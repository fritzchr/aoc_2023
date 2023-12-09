fn extract_sequences(input: &str) -> Vec<Vec<i64>> {
    let mut sequences: Vec<Vec<i64>> = vec![];
    for line in input.lines() {
        let sequence: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        sequences.push(sequence);
    }
    sequences
}

fn calculate_next_sequence(sequence: Vec<i64>) -> Option<Vec<i64>> {
    let mut new_sequence: Vec<i64> = vec![];

    let mut finished = true;
    for number in &sequence {
        if *number != 0 {
            finished = false;
            break;
        }
    }

    if finished {
        return None;
    }

    for i in 0..sequence.len() {
        if sequence.len() <= i + 1 {
            break;
        }

        let next = i + 1;
        new_sequence.push(sequence[next] - sequence[i]);
    }
    Some(new_sequence)
}

fn assignment01(input: &str) -> i64 {
    let sequences = extract_sequences(input);
    let mut sum = 0;

    for sequence in sequences {
        let mut current_sequence = Some(sequence);
        let mut sequence_history: Vec<Vec<i64>> = vec![];

        while let Some(next_sequence) = current_sequence {
            sequence_history.push(next_sequence.clone());
            if let Some(new_sequence) = calculate_next_sequence(next_sequence) {
                current_sequence = Some(new_sequence);
            } else {
                current_sequence = None;
            }
        }

        for history in sequence_history.iter().rev() {
            sum += history.last().unwrap();
        }
    }
    sum
}

fn assignment02(input: &str) -> u32 {
    0
}

pub fn day09() {
    let input = aoc_2023::read_input("input/day09.txt").expect("Failed to read file");

    let result_01 = assignment01(&input);
    let result_02 = assignment02(&input);

    println!("Day 09 - Assignment 01 solution: {result_01}");
    println!("Day 09 - Assignment 02 solution: {result_02}");
    println!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn assignment01_test() {
        assert_eq!(assignment01(&EXAMPLE_DATA.to_owned()), 114);
    }

    // #[test]
    // fn assignment02_test() {
    //     assert_eq!(assignment02(&EXAMPLE_DATA.to_owned()), 71503);
    // }
}
