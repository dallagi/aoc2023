Template for daily solution:

```rust
use std::fs;

fn day1(input: &str) -> u32 {
    todo!()
}

fn day2(input: &str) -> u32 {
    todo!()
}

fn main() {
    let input = fs::read_to_string("src/bin/input2.txt").unwrap();

    println!("Answer to day2 part 1: {}", day1(&input));
    println!("Answer to day2 part 2: {}", day2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let example_input = r#""#;

        assert_eq!(142, day1(example_input));
    }

    #[test]
    fn test_example_2() {
        let example_input = r#""#;

        assert_eq!(281, day2(example_input));
    }
}
```
