use std::fs;
use std::str::Lines;

fn part1(input: &str) -> u64 {
    Almanac::parse(input).lowest_location()
}

fn part2(input: &str) -> u64 {
    todo!()
}

fn main() {
    let input = fs::read_to_string("src/bin/input5.txt").unwrap();

    println!("Answer to day5 part 1: {}", part1(&input));
    println!("Answer to day5 part 2: {}", part2(&input));
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn parse(input: &str) -> Self {
        let mut blocks = input.split("\n\n");

        let seeds_line = blocks.next().unwrap();
        let seeds = seeds_line
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|num| num.parse().unwrap())
            .collect();

        let maps = blocks.map(AlmanacMap::parse).collect();

        Self { seeds, maps }
    }

    fn lowest_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| {
                self.maps
                    .iter()
                    .fold(*seed, |acc, almanac_map| almanac_map.map(acc))
            })
            .min()
            .unwrap()
    }
}

#[derive(Debug, Default)]
struct AlmanacMap {
    ranges_sources_start: Vec<u64>,
    ranges_dests_start: Vec<u64>,
    ranges_length: Vec<u64>,
}

impl AlmanacMap {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        lines.next();

        let mut res = Self::default();
        for line in lines {
            let mut nums = line.splitn(3, ' ').map(|num| num.parse().unwrap());

            res.ranges_dests_start.push(nums.next().unwrap());
            res.ranges_sources_start.push(nums.next().unwrap());
            res.ranges_length.push(nums.next().unwrap());
        }

        res
    }

    fn map(&self, num: u64) -> u64 {
        for i in 0..self.ranges_sources_start.len() {
            let range_source_start = self.ranges_sources_start[i];
            let range_dest_start = self.ranges_dests_start[i];
            let range_length = self.ranges_length[i];

            if (range_source_start..(range_source_start + range_length)).contains(&num) {
                return range_dest_start + (num - range_source_start);
            }
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_almanac_map() {
        let example_map_input = r#"seed-to-soil map:
50 98 2
52 50 48
"#;

        let almanac_map = AlmanacMap::parse(example_map_input);

        assert_eq!(53, almanac_map.map(51));
        assert_eq!(100, almanac_map.map(100));
    }

    #[test]
    fn test_parser() {
        let example_input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

        assert!(!Almanac::parse(example_input)
            .maps
            .first()
            .unwrap()
            .ranges_sources_start
            .is_empty())
    }

    #[test]
    fn test_part_1() {
        let example_input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

        assert_eq!(35, part1(example_input));
    }

    // #[test]
    // fn test_part_2() {
    //     let example_input = r#""#;

    //     assert_eq!(281, part2(example_input));
    // }
}
