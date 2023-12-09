Template for daily solution:

```rust
use std::fs;

fn part1(input: &str) -> u64 {
    todo!()
}

fn part2(input: &str) -> u64 {
    todo!()
}

fn main() {
    let input = fs::read_to_string("src/bin/input2.txt").unwrap();

    println!("Answer to day2 part 1: {}", part1(&input));
    println!("Answer to day2 part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let example_input = r#""#;

        assert_eq!(142, part1(example_input));
    }

    #[test]
    fn test_part_2() {
        let example_input = r#""#;

        assert_eq!(281, part2(example_input));
    }
}
```
