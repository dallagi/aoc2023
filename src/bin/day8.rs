use std::collections::HashMap;
use std::fs;

use regex::Regex;

fn part1(input: &str) -> u64 {
    Map::parse(input).required_steps("AAA", "ZZZ")
}

fn part2(input: &str) -> u32 {
    todo!()
}

fn main() {
    let input = fs::read_to_string("src/bin/input8.txt").unwrap();

    println!("Answer to day8 part 1: {}", part1(&input));
    println!("Answer to day8 part 2: {}", part2(&input));
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(Direction::from_char)
            .collect();

        lines.next();
        let node_regex =
            Regex::new(r"(?P<id>[A-Z]{3}) = \((?P<left>[A-Z]{3}), (?P<right>[A-Z]{3})\)").unwrap();
        let nodes = lines
            .map(|line| {
                let captures = node_regex.captures(line).unwrap();
                (
                    captures.name("id").unwrap().as_str().to_owned(),
                    (
                        captures.name("left").unwrap().as_str().to_owned(),
                        captures.name("right").unwrap().as_str().to_owned(),
                    ),
                )
            })
            .collect();

        Self {
            instructions,
            nodes,
        }
    }

    fn required_steps(&self, from: &str, destination: &str) -> u64 {
        let mut instructions = self.instructions.iter().cycle();
        let mut current_location = from;
        let mut steps = 0;

        while current_location != destination {
            steps += 1;

            let (left, right) = self.nodes.get(current_location).unwrap();
            current_location = match instructions.next().unwrap() {
                Direction::Left => left,
                Direction::Right => right,
            };
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let example_input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

        assert_eq!(2, part1(example_input));
    }

    #[test]
    fn test_part_1_second_example() {
        let example_input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

        assert_eq!(6, part1(example_input));
    }

    #[test]
    fn test_part_2() {
        let example_input = r#""#;

        assert_eq!(281, part2(example_input));
    }
}