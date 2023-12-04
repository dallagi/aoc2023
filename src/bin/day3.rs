use std::fs;
use std::ops::{Mul, RangeInclusive};

fn part1(input: &str) -> u32 {
    EngineSchematic::parse(input)
        .engine_parts()
        .iter()
        .map(|number| number.value)
        .sum()
}

fn part2(input: &str) -> u32 {
    EngineSchematic::parse(input).gears().iter().sum()
}

fn main() {
    let input = fs::read_to_string("src/bin/input3.txt").unwrap();

    println!("Answer to day3 part 1: {}", part1(&input));
    println!("Answer to day3 part 2: {}", part2(&input));
}

#[derive(Default, Debug)]
struct EngineSchematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl EngineSchematic {
    fn parse(input: &str) -> Self {
        let mut result = EngineSchematic::default();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                if c.is_digit(10) {
                    if let Some(last_num) = result.numbers.last_mut() {
                        // new digit for existing number
                        if *last_num.coord_x.end() == x.saturating_sub(1) {
                            last_num.value = last_num.value * 10 + c.to_digit(10).unwrap();
                            last_num.coord_x = *last_num.coord_x.start()..=x;
                            continue;
                        }
                    }
                    // new number
                    result.numbers.push(Number {
                        value: c.to_digit(10).unwrap(),
                        coord_x: (x..=x),
                        coord_y: y,
                    })
                } else {
                    // new symbol
                    result.symbols.push(Symbol {
                        value: c,
                        coord_x: x,
                        coord_y: y,
                    })
                }
            }
        }
        result
    }

    fn engine_parts(&self) -> Vec<&Number> {
        self.numbers
            .iter()
            .filter(|number| {
                self.symbols
                    .iter()
                    .any(|symbol| number.is_close_to(symbol.coord_x, symbol.coord_y))
            })
            .collect()
    }

    fn gears(&self) -> Vec<u32> {
        self.symbols
            .iter()
            .filter(|symbol| symbol.value == '*')
            .map(|symbol| self.numbers_around_symbol_at(symbol.coord_x, symbol.coord_y))
            .filter_map(|numbers: Vec<&Number>| {
                if numbers.len() >= 2 {
                    let gear_ratio = numbers.iter().map(|number| number.value).reduce(Mul::mul);
                    Some(gear_ratio.unwrap())
                } else {
                    None
                }
            })
            .collect()
    }

    fn numbers_around_symbol_at<'a>(&'a self, coord_x: usize, coord_y: usize) -> Vec<&'a Number> {
        self.numbers
            .iter()
            .filter(|number| number.is_close_to(coord_x, coord_y))
            .collect()
    }
}

#[derive(Debug)]
struct Number {
    value: u32,
    coord_x: RangeInclusive<usize>,
    coord_y: usize,
}

impl Number {
    fn is_close_to(&self, coord_x: usize, coord_y: usize) -> bool {
        let x_range = (self.coord_x.start().saturating_sub(1))..=(self.coord_x.end() + 1);
        let y_range = (self.coord_y.saturating_sub(1))..=(self.coord_y + 1);

        x_range.contains(&coord_x) && y_range.contains(&coord_y)
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    coord_x: usize,
    coord_y: usize,
}

impl Symbol {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let example_input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

        assert_eq!(4361, part1(example_input));
    }

    #[test]
    fn test_part_2() {
        let example_input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

        assert_eq!(467835, part2(example_input));
    }
}
