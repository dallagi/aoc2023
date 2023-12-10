use std::collections::{HashMap, HashSet};
use std::fs;

fn part1(input: &str) -> u64 {
    PipesGraph::parse(input).max_distance_in_loop()
}

fn part2(input: &str) -> u64 {
    PipesGraph::parse(input).points_within_loop()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    fn north(self) -> Option<Self> {
        if self.y == 0 {
            return None;
        }
        Some(Self::new(self.x, self.y - 1))
    }

    fn south(self) -> Option<Self> {
        Some(Self::new(self.x, self.y + 1))
    }

    fn east(self) -> Option<Self> {
        Some(Self::new(self.x + 1, self.y))
    }

    fn west(self) -> Option<Self> {
        if self.x == 0 {
            return None;
        }
        Some(Self::new(self.x - 1, self.y))
    }
}

#[derive(Debug)]
struct PipesGraph {
    adj_list: HashMap<Point, Vec<Point>>,
    start: Point,
}

impl PipesGraph {
    fn parse(input: &str) -> Self {
        let mut adj_list = HashMap::new();
        let mut start = Point::new(0, 0);

        let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        for (y, line) in chars.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                let loc = Point::new(x as u64, y as u64);
                let adjacents = match char {
                    '.' => vec![],
                    '|' => vec![loc.north(), loc.south()],
                    '-' => vec![loc.west(), loc.east()],
                    'L' => vec![loc.north(), loc.east()],
                    'J' => vec![loc.north(), loc.west()],
                    '7' => vec![loc.south(), loc.west()],
                    'F' => vec![loc.south(), loc.east()],
                    'S' => {
                        start = loc;
                        vec![loc.north(), loc.west(), loc.south(), loc.east()]
                    }
                    _ => panic!(),
                };

                adj_list.insert(loc, adjacents.iter().filter_map(|p| *p).collect());
            }
        }

        Self { adj_list, start }
    }

    fn max_distance_in_loop(&self) -> u64 {
        self.find_loop().len() as u64 / 2
    }

    fn find_loop(&self) -> HashSet<Point> {
        let mut visited = HashSet::new();
        visited.insert(self.start);

        self.adj_list
            .get(&self.start)
            .unwrap()
            .iter()
            .find(|start_neighbor| !self.adj_list.get(&start_neighbor).unwrap().is_empty())
            .map(|start_neighbor| {
                self.find_loop_dfs(*start_neighbor, self.start, 1, visited.clone())
            })
            .unwrap()
    }

    fn find_loop_dfs(
        &self,
        current_location: Point,
        previous_location: Point,
        steps: u64,
        mut visited: HashSet<Point>,
    ) -> HashSet<Point> {
        if visited.contains(&current_location) {
            return visited;
        }
        visited.insert(current_location);

        let maybe_next_location = self
            .adj_list
            .get(&current_location)
            .unwrap()
            .iter()
            .find(|loc| **loc != previous_location);

        if let Some(next_location) = maybe_next_location {
            self.find_loop_dfs(*next_location, current_location, steps + 1, visited)
        } else {
            HashSet::new()
        }
    }

    fn points_within_loop(&self) -> u64 {
        let loop_members = self.find_loop();
        let mut count = 0;

        for point in self.adj_list.keys() {
            if loop_members.contains(point) {
                continue;
            }

            let x_barriers_left_count = loop_members
                .iter()
                .filter(|p| {
                    point.x < p.x
                        && p.y == point.y
                        && (self
                            .adj_list
                            .get(p)
                            .unwrap()
                            .contains(&p.north().unwrap_or(Point::new(999999999, 999999999)))
                            || self.adj_list.get(p).unwrap().contains(&p.south().unwrap()))
                })
                .count();
            let x_barriers_right_count = loop_members
                .iter()
                .filter(|p| {
                    point.x > p.x
                        && p.y == point.y
                        && (self
                            .adj_list
                            .get(p)
                            .unwrap()
                            .contains(&p.north().unwrap_or(Point::new(999999999, 999999999)))
                            || self.adj_list.get(p).unwrap().contains(&p.south().unwrap()))
                })
                .count();
            let y_barriers_top_count = loop_members
                .iter()
                .filter(|p| {
                    point.y > p.y
                        && p.x == point.x
                        && (self.adj_list.get(p).unwrap().contains(&p.east().unwrap())
                            || self.adj_list.get(p).unwrap().contains(&p.west().unwrap()))
                })
                .count();
            let y_barriers_bottom_count = loop_members
                .iter()
                .filter(|p| {
                    point.y < p.y
                        && p.x == point.x
                        && (self.adj_list.get(p).unwrap().contains(&p.east().unwrap())
                            || self.adj_list.get(p).unwrap().contains(&p.west().unwrap()))
                })
                .count();

            // if !((x_barriers_left_count % 2 == 0)
            //     && (x_barriers_right_count % 2 == 0)
            //     && (y_barriers_bottom_count % 2 == 0)
            //     && (y_barriers_top_count % 2 == 0))
            let is_inside = (((x_barriers_left_count as i64 - x_barriers_right_count as i64) % 2
                != 0)
                || ((y_barriers_top_count as i64 - y_barriers_bottom_count as i64) % 2 != 0));

            if is_inside {
                count += 1;
                continue;
            }

            // if (x_barriers_left_count % 2 != 0 || x_barriers_right_count % 2 != 0)
            //     && x_barriers_left_count != x_barriers_right_count
            // {
            //     count += 1;
            //     continue;
            // }
            // if (y_barriers_top_count % 2 != 0 || y_barriers_bottom_count % 2 != 0)
            //     && y_barriers_top_count != y_barriers_bottom_count
            // {
            //     count += 1;
            //     continue;
            // }
        }
        return count;
    }
}

fn main() {
    let input = fs::read_to_string("src/bin/input10.txt").unwrap();

    println!("Answer to day10 part 1: {}", part1(&input));
    println!("Answer to day10 part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_simple() {
        let example_input = r#".....
.S-7.
.|.|.
.L-J.
.....
"#;

        assert_eq!(4, part1(example_input));
    }

    #[test]
    fn test_part_1() {
        let example_input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

        assert_eq!(8, part1(example_input));
    }

    #[test]
    fn test_part_2_simple_1() {
        let example_input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;

        assert_eq!(4, part2(example_input));
    }

    #[test]
    fn test_part_2_simple_2() {
        let example_input = r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
"#;

        assert_eq!(4, part2(example_input));
    }

    #[test]
    fn test_part_2_standard() {
        let example_input = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;

        assert_eq!(8, part2(example_input));
    }

    #[test]
    fn test_part_2_comples() {
        let example_input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;

        assert_eq!(10, part2(example_input));
    }
}
