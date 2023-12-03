use std::fs;

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
                        if x > 0 && last_num.coord_end == (x - 1, y) {
                            last_num.value = last_num.value * 10 + c.to_digit(10).unwrap();
                            last_num.coord_end = (x, y);
                            continue;
                        }
                    }
                    result.numbers.push(Number {
                        value: c.to_digit(10).unwrap(),
                        coord_start: (x, y),
                        coord_end: (x, y),
                    })
                } else {
                    result.symbols.push(Symbol {
                        value: c,
                        coord: (x, y),
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
                    .any(|symbol| number.is_close_to(&symbol))
            })
            .collect()
    }

    fn gears(&self) -> Vec<u32> {
        self.symbols
            .iter()
            .filter(|symbol| symbol.value == '*')
            .map(|symbol| {
                self.numbers
                    .iter()
                    .filter(|number| number.is_close_to(symbol))
                    .collect()
            })
            .filter_map(|numbers: Vec<&Number>| {
                if numbers.len() >= 2 {
                    let gear_ratio = numbers
                        .iter()
                        .map(|number| number.value)
                        .reduce(|a, b| a * b);
                    Some(gear_ratio.unwrap())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct Number {
    value: u32,
    coord_start: (usize, usize),
    coord_end: (usize, usize),
}

impl Number {
    fn is_close_to(&self, symbol: &Symbol) -> bool {
        let x_range = (self.coord_start.0.saturating_sub(1))..=(self.coord_end.0 + 1);
        let y_range = (self.coord_start.1.saturating_sub(1))..=(self.coord_start.1 + 1);

        x_range.contains(&symbol.coord.0) && y_range.contains(&symbol.coord.1)
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    coord: (usize, usize),
}

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
