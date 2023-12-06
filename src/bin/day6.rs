use std::fs;

fn part1(input: &str) -> u32 {
    Games::parse(input).combinations()
}

fn part2(input: &str) -> u32 {
    todo!()
}

#[derive(Debug)]
struct Games {
    times: Vec<u32>,
    distances: Vec<u32>,
}

impl Games {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let times: Vec<u32> = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|num| num.trim().parse().unwrap())
            .collect();
        let distances: Vec<u32> = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|num| num.trim().parse().unwrap())
            .collect();

        Games { times, distances }
    }

    fn combinations(&self) -> u32 {
        self.times
            .iter()
            .zip(self.distances.iter())
            .map(|(time, distance)| {
                Self::max_press_time(*distance, *time) - Self::min_press_time(*distance, *time) + 1
            })
            .reduce(|a, b| a * b)
            .unwrap()
    }

    fn min_press_time(record: u32, race_duration: u32) -> u32 {
        let mut from_time = 0;
        let mut to_time = race_duration;

        loop {
            let middle = from_time + (to_time - from_time) / 2;
            let this_distance = Self::distance(middle, race_duration);
            let prev_distance = Self::distance(middle.saturating_sub(1), race_duration);

            if from_time > to_time {
                panic!()
            }
            match (this_distance, prev_distance) {
                (td, pd) if td <= record && pd <= record => {
                    from_time = middle;
                    continue;
                }
                (td, pd) if td > record && pd > record => {
                    to_time = middle;
                    continue;
                }
                (td, pd) if td > record && pd <= record => {
                    return middle;
                }
                _ => panic!(),
            }
        }
    }

    fn max_press_time(record: u32, race_duration: u32) -> u32 {
        let mut from_time = 0;
        let mut to_time = race_duration;

        loop {
            let middle = from_time + (to_time - from_time) / 2;
            let this_distance = Self::distance(middle, race_duration);
            let next_distance = Self::distance(middle + 1, race_duration);

            if from_time > to_time {
                panic!()
            }
            match (this_distance, next_distance) {
                (td, nd) if td <= record && nd <= record => {
                    to_time = middle;
                    continue;
                }
                (td, nd) if td > record && nd > record => {
                    from_time = middle;
                    continue;
                }
                (td, nd) if td > record && nd <= record => {
                    return middle;
                }
                _ => panic!(),
            }
        }
    }

    fn distance(press_time: u32, race_duration: u32) -> u32 {
        press_time * (race_duration - press_time)
    }
}

fn main() {
    let input = fs::read_to_string("src/bin/input6.txt").unwrap();

    println!("Answer to day6 part 1: {}", part1(&input));
    println!("Answer to day6 part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_press_time() {
        assert_eq!(2, Games::min_press_time(9, 7));
    }

    #[test]
    fn test_max_press_time() {
        assert_eq!(5, Games::max_press_time(9, 7));
    }

    #[test]
    fn test_simple() {
        assert_eq!(
            4,
            Games {
                times: vec![7],
                distances: vec![9]
            }
            .combinations()
        )
    }

    #[test]
    fn test_part_1() {
        let example_input = r#"Time:      7  15   30
Distance:  9  40  200
"#;

        assert_eq!(288, part1(example_input));
    }

    // #[test]
    // fn test_part_2() {
    //     let example_input = r#""#;

    //     assert_eq!(281, part2(example_input));
    // }
}
