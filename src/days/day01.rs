fn find_first_and_last_digit_01(text: &str) -> (Option<char>, Option<char>) {
    let mut first = None;
    let mut last = None;

    for char in text.chars() {
        if char.is_numeric() {
            if first.is_none() {
                first = Some(char);
            }
            last = Some(char);
        }
    }
    (first, last)
}

fn find_first_and_last_digit_02(text: &str) -> (Option<char>, Option<char>) {
    // We can't simply replace for example "one" with "1", since that would
    // parse the following number incorrectly.
    // Therefore, we make sure the beginng and ending of each number is still correct.
    //
    // For example "4nineight" -> "49ight", which is wrong.
    // With this approach "4n9eight", "eight" will still be correctly parsed.
    let line = text
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    find_first_and_last_digit_01(line.as_str())
}

fn get_calibration_value(
    text: &str,
    find_digit_func: fn(&str) -> (Option<char>, Option<char>),
) -> Option<u32> {
    let (first, last) = find_digit_func(&text);
    match (first, last) {
        (Some(first), Some(last)) => match (first.to_digit(10), last.to_digit(10)) {
            (Some(first_digit), Some(last_digit)) => {
                let calibartion_value = 10 * first_digit + last_digit;
                Some(calibartion_value)
            }
            _ => {
                eprintln!("Failed to convert char to digit");
                None
            }
        },
        _ => {
            eprintln!("Failed to get a number");
            None
        }
    }
}

fn assignment_01(input: &String) -> u32 {
    let mut sum = 0;

    for entry in input.lines() {
        match get_calibration_value(&entry, find_first_and_last_digit_01) {
            Some(value) => sum += value,
            None => panic!("Expected a value"),
        };
    }
    sum
}

fn assignment_02(input: &String) -> u32 {
    let mut sum = 0;

    for entry in input.lines() {
        match get_calibration_value(&entry, find_first_and_last_digit_02) {
            Some(value) => {
                println!("{entry} -> {value}");
                sum += value
            }
            None => panic!("Expected a value"),
        };
    }
    sum
}

pub fn day01() {
    let input = aoc_2023::read_input("input/day01.txt").expect("Failed to read file");

    let result_01 = assignment_01(&input);
    let result_02 = assignment_02(&input);

    println!("Day 01 - Assignment 01 solution: {result_01}");
    println!("Day 01 - Assignment 02 solution: {result_02}");
    println!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA1: &'static str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const EXAMPLE_DATA2: &'static str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn assignment01_test() {
        assert_eq!(assignment_01(&EXAMPLE_DATA1.to_owned()), 142);
    }

    #[test]
    fn assignment02_test() {
        assert_eq!(assignment_02(&EXAMPLE_DATA2.to_owned()), 281);
    }
}
