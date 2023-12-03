use std::fs;

fn part1(input: &str) -> u32 {
    EngineSchematic::parse(input)
        .valid_numbers()
        .iter()
        .map(|number| number.value)
        .sum()
}

fn part2(input: &str) -> u32 {
    todo!()
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

    fn valid_numbers(&self) -> Vec<&Number> {
        self.numbers
            .iter()
            .filter(|number| {
                let x_range = (number.coord_start.0.saturating_sub(1))..=(number.coord_end.0 + 1);
                let y_range = (number.coord_start.1.saturating_sub(1))..=(number.coord_start.1 + 1);
                self.symbols.iter().any(|symbol| {
                    x_range.contains(&symbol.coord.0) && y_range.contains(&symbol.coord.1)
                })
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

    // #[test]
    // fn test_part_2() {
    //     let example_input = r#""#;

    //     assert_eq!(281, part2(example_input));
    // }
}
