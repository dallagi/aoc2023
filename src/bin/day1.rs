use std::fs;

const DIGITS: &[&'static str] = &[
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn day1(input: &str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let first_digit = line.chars().find_map(|char| char.to_digit(10)).unwrap();
        let second_digit = line
            .chars()
            .rev()
            .find_map(|char| char.to_digit(10))
            .unwrap();
        sum += first_digit * 10 + second_digit;
    }
    sum
}

fn day2(input: &str) -> u32 {
    input.lines().into_iter().map(line_to_number).sum()
}

fn line_to_number(line: &str) -> u32 {
    let (first_digit, _) = DIGITS
        .iter()
        .map(|digit| line.find(digit).map(|pos| (digit, pos)))
        .flatten()
        .min_by_key(|&(_digit, pos)| pos)
        .unwrap();

    let (last_digit, _) = DIGITS
        .iter()
        .map(|digit| line.rfind(digit).map(|pos| (digit, pos)))
        .flatten()
        .max_by_key(|&(_digit, pos)| pos)
        .unwrap();

    parse_digit(first_digit) * 10 + parse_digit(last_digit)
}

fn parse_digit(digit: &str) -> u32 {
    if let Ok(value) = digit.parse() {
        return value;
    }

    match digit {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!(),
    }
}

fn main() {
    let input = fs::read_to_string("src/bin/input1.txt").unwrap();

    println!("Answer to day1 part 1: {}", day1(&input));
    println!("Answer to day1 part 2: {}", day2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let example_input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        assert_eq!(142, day1(example_input));
    }

    #[test]
    fn test_example_2() {
        let example_input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        assert_eq!(281, day2(example_input));
    }

    #[test]
    fn test_parses_overlapping_digits() {
        let example_input = r#"eighthree"#;

        assert_eq!(83, line_to_number(example_input));
    }
}
