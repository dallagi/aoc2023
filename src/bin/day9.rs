use std::fs;

fn part1(input: &str) -> i64 {
    parse_input(input).iter().map(Sequence::predict_next).sum()
}

fn part2(input: &str) -> i64 {
    parse_input(input)
        .iter()
        .map(Sequence::predict_previous)
        .sum()
}

fn main() {
    let input = fs::read_to_string("src/bin/input9.txt").unwrap();

    println!("Answer to day9 part 1: {}", part1(&input));
    println!("Answer to day9 part 2: {}", part2(&input));
}

struct Sequence(Vec<i64>);

impl Sequence {
    fn parse(line: &str) -> Self {
        Sequence(line.split(" ").map(|num| num.parse().unwrap()).collect())
    }

    fn predict_next(&self) -> i64 {
        self.differences()
            .iter()
            .rfold(0, |last_prediction, diffs_sequence| {
                last_prediction + diffs_sequence.last().unwrap()
            })
    }

    fn predict_previous(&self) -> i64 {
        self.differences()
            .iter()
            .rfold(0, |last_prediction, diffs_sequence| {
                diffs_sequence.first().unwrap() - last_prediction
            })
    }

    fn differences(&self) -> Vec<Vec<i64>> {
        let mut res = vec![self.0.clone()];

        loop {
            let last_seq = res.last().unwrap();
            if Self::all_zeroes(&last_seq) {
                return res;
            }
            res.push(
                last_seq
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect(),
            );
        }
    }

    fn all_zeroes(seq: &Vec<i64>) -> bool {
        seq.iter().all(|n| *n == 0)
    }
}

fn parse_input(input: &str) -> Vec<Sequence> {
    input.lines().map(Sequence::parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let example_input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

        assert_eq!(114, part1(example_input));
    }

    #[test]
    fn test_part_2() {
        let example_input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

        assert_eq!(2, part2(example_input));
    }
}
