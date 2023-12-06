fn extract_multiple_races(input: &str) -> (Vec<u32>, Vec<u32>) {
    let lines: Vec<&str> = input.lines().collect();

    let duration: Vec<u32> = lines[0]
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let distance: Vec<u32> = lines[1]
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    assert_eq!(duration.len(), distance.len());

    (duration, distance)
}

fn calculate_win_possibilities_per_race(input: &str) -> Vec<u32> {
    let (duration, distance) = extract_multiple_races(input);

    let mut possible_wins_per_race: Vec<u32> = vec![];

    for i in 0..duration.len() {
        let duration = duration[i];
        let distance = distance[i];
        let mut possible_wins_for_race = 0;
        for j in 0..duration {
            let remaining_duration = duration - j;
            let distance_traveled = j * remaining_duration;

            if distance_traveled > distance {
                possible_wins_for_race += 1;
            }
        }
        possible_wins_per_race.push(possible_wins_for_race);
    }

    possible_wins_per_race
}

fn extract_single_race(input: &str) -> (u64, u64) {
    let lines: Vec<&str> = input.lines().collect();

    let duration_str: String = lines[0]
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .map(|s| s.to_string())
        .collect();

    let distance_str: String = lines[1]
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .map(|s| s.to_string())
        .collect();

    let duration = duration_str.parse::<u64>().expect("Failed to parse value");
    let distance = distance_str.parse::<u64>().expect("Failed to parse value");

    (duration, distance)
}

fn calculate_win_possibilities_for_race(input: &str) -> u64 {
    let (duration, distance) = extract_single_race(input);

    let mut possible_wins_per_race: u64 = 0;

    for i in 0..duration {
        let remaining_duration = duration - i;
        let distance_traveled = i * remaining_duration;

        if distance_traveled > distance {
            possible_wins_per_race += 1;
        }
    }

    possible_wins_per_race
}

fn assignment01(input: &str) -> u32 {
    let possible_wins_per_race = calculate_win_possibilities_per_race(input);

    let mut sum = 0;
    for race in possible_wins_per_race {
        sum = if sum == 0 { race } else { sum * race }
    }

    sum
}

fn assignment02(input: &str) -> u64 {
    calculate_win_possibilities_for_race(input)
}

pub fn day06() {
    let input = aoc_2023::read_input("input/day06.txt").expect("Failed to read file");

    let result_01 = assignment01(&input);
    let result_02 = assignment02(&input);

    println!("Day 06 - Assignment 01 solution: {result_01}");
    println!("Day 06 - Assignment 02 solution: {result_02}");
    println!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = r"Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn assignment01_test() {
        assert_eq!(assignment01(&EXAMPLE_DATA.to_owned()), 288);
    }

    #[test]
    fn assignment02_test() {
        assert_eq!(assignment02(&EXAMPLE_DATA.to_owned()), 71503);
    }
}
