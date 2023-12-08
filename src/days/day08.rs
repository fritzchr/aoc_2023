use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Node {
    current: String,
    left: String,
    right: String,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current
    }
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_it = s.split_terminator('=');
        let current = line_it.next().unwrap_or_default().trim().to_owned();
        let rest = line_it.last().unwrap_or_default().trim();

        let mut node_it = rest.split_terminator(',');

        let mut left = node_it.next().unwrap_or_default().trim();
        if left.contains('(') {
            left = left.strip_prefix('(').unwrap();
        }

        let mut right = node_it.last().unwrap_or_default().trim();
        if right.contains(')') {
            right = right.strip_suffix(')').unwrap();
        }

        Ok(Self {
            current,
            left: left.to_owned(),
            right: right.to_owned(),
        })
    }
}

fn get_directions(line: &str) -> Vec<char> {
    let mut directions: Vec<char> = vec![];
    for char in line.chars() {
        if char.is_alphabetic() {
            directions.push(char);
        }
    }
    directions
}

fn get_nodes(lines: &str) -> HashMap<String, Node> {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in lines.lines() {
        if !line.contains('=') {
            continue;
        }
        if let Ok(node) = Node::from_str(line) {
            nodes.insert(node.current.to_owned(), node);
        }
    }
    nodes
}

fn assignment01(input: &str) -> usize {
    let direction_str = input.lines().next().expect("Invalid input");
    let directions = get_directions(direction_str);
    let all_nodes = get_nodes(&input);

    let mut i = 0;
    let mut path: Vec<&Node> = vec![];
    let mut curr_node: &Node = all_nodes.get("AAA").expect("There must be a starting node");
    let mut nodes_traversed = 0;

    path.push(curr_node);

    loop {
        if i >= directions.len() {
            // restart from the beginning
            i = 0;
        }
        let direction = directions.get(i).expect("Expected a direction");
        if *direction == 'L' {
            curr_node = all_nodes.get(curr_node.left.as_str()).unwrap();
        } else if *direction == 'R' {
            curr_node = all_nodes.get(curr_node.right.as_str()).unwrap();
        }

        if path.contains(&curr_node) {
            while path.contains(&curr_node) {
                path.pop();
            }
        } else {
            path.push(curr_node);
        }

        nodes_traversed += 1;

        if let Some(node) = path.last() {
            if node.current == "ZZZ" {
                break;
            }
        }
        i += 1;
    }
    nodes_traversed
}

fn assignment02(input: &str) -> u32 {
    0
}

pub fn day08() {
    let input = aoc_2023::read_input("input/day08.txt").expect("Failed to read file");

    let result_01 = assignment01(&input);
    let result_02 = assignment02(&input);

    println!("Day 08 - Assignment 01 solution: {result_01}");
    println!("Day 08 - Assignment 02 solution: {result_02}");
    println!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn assignment01_test() {
        assert_eq!(assignment01(&EXAMPLE_DATA.to_owned()), 6);
    }

    // #[test]
    // fn assignment02_test() {
    //     assert_eq!(assignment02(&EXAMPLE_DATA.to_owned()), 71503);
    // }
}
