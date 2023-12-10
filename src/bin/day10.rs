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
    letters: HashMap<Point, char>,
    max_x: u64,
    max_y: u64,
}

impl PipesGraph {
    fn parse(input: &str) -> Self {
        let mut adj_list: HashMap<Point, Vec<Point>> = HashMap::new();
        let mut start = Point::new(0, 0);
        let mut letters = HashMap::new();

        let chars: Vec<Vec<char>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();
        let max_y = chars.len() as u64 - 1;
        let max_x = chars.first().unwrap().len() as u64 - 1;

        for (y, line) in chars.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                let loc = Point::new(x as u64, y as u64);
                letters.insert(loc, *char);

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
                        vec![]
                    }
                    _ => panic!(),
                };

                adj_list.insert(loc, adjacents.iter().filter_map(|p| *p).collect());
            }
        }

        let mut start_neighbors = Vec::new();
        for x in start.x.saturating_sub(1)..=(start.x + 1) {
            for y in start.y.saturating_sub(1)..=(start.y + 1) {
                let neighbor = Point::new(x, y);
                if adj_list.get(&neighbor).unwrap().contains(&start) {
                    start_neighbors.push(neighbor);
                }
            }
        }
        adj_list.insert(start, start_neighbors);

        Self {
            adj_list,
            start,
            letters,
            max_x,
            max_y,
        }
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

    fn print_subset(
        &self,
        loop_points: &HashSet<Point>,
        points_to_highlight: &HashSet<Point>,
        fancy: bool,
    ) {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let loc = Point::new(x, y);
                let letter = self.letters.get(&loc).unwrap().to_string();
                let styled = match letter.as_str() {
                    "L" => "└",
                    "J" => "┘",
                    "-" => "─",
                    "|" => "│",
                    "F" => "┌",
                    "7" => "┐",

                    other => other,
                };

                if points_to_highlight.contains(&loc) && loop_points.contains(&loc) {
                    print!("?");
                } else if points_to_highlight.contains(&loc) {
                    print!("*");
                } else if loop_points.contains(&loc) {
                    if fancy {
                        print!("{}", styled);
                    } else {
                        print!("{}", letter);
                    }
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }

    fn points_within_loop(&self) -> u64 {
        let loop_members = self.find_loop();
        let mut sorted_loop_members: Vec<Point> = loop_members.iter().map(|x| x.clone()).collect();
        sorted_loop_members.sort_by_key(|point| (point.x, point.y));

        // self.print_subset(&loop_members, &HashSet::new(), false);
        let mut count = 0;
        let mut points_inside = HashSet::new();

        for point in self.adj_list.keys() {
            if loop_members.contains(point) {
                continue;
            }

            let x_barriers_left: Vec<String> = sorted_loop_members
                .iter()
                .filter(|p| point.x < p.x)
                .filter(|p| point.y == p.y)
                .map(|p| *self.letters.get(p).unwrap())
                .filter(|c| *&['|', 'S', '7', 'F', 'J', 'L'].contains(c))
                .map(|c| c.to_string())
                .collect();

            let x_barriers_left = x_barriers_left
                .join("")
                .replace("FJ", "|")
                .replace("L7", "|");

            let x_barriers_right: Vec<String> = sorted_loop_members
                .iter()
                .filter(|p| point.x < p.x)
                .filter(|p| point.y == p.y)
                .map(|p| *self.letters.get(p).unwrap())
                .filter(|c| *&['|', 'S', '7', 'F', 'J', 'L'].contains(c))
                .map(|c| c.to_string())
                .collect();

            let x_barriers_right = x_barriers_right
                .join("")
                .replace("FJ", "|")
                .replace("L7", "|");

            let y_barriers_top: Vec<String> = sorted_loop_members
                .iter()
                .filter(|p| point.y > p.y)
                .filter(|p| point.x == p.x)
                .map(|p| *self.letters.get(p).unwrap())
                .filter(|c| *&['-', 'S', '7', 'F', 'J', 'L'].contains(c))
                .map(|c| c.to_string())
                .collect();

            let y_barriers_top = y_barriers_top
                .join("")
                .replace("7L", "-")
                .replace("FJ", "-");

            let y_barriers_bottom: Vec<String> = sorted_loop_members
                .iter()
                .filter(|p| point.y < p.y)
                .filter(|p| point.x == p.x)
                .map(|p| *self.letters.get(p).unwrap())
                .filter(|c| *&['-', 'S', '7', 'F', 'J', 'L'].contains(c))
                .map(|c| c.to_string())
                .collect();

            let y_barriers_bottom = y_barriers_bottom
                .join("")
                .replace("7L", "-")
                .replace("FJ", "-");

            // on border
            if (x_barriers_left.is_empty())
                || (x_barriers_right.is_empty())
                || (y_barriers_bottom.is_empty())
                || (y_barriers_top.is_empty())
            {
                continue;
            }

            let is_inside = !((x_barriers_left.len() % 2 == 0)
                && (x_barriers_right.len() % 2 == 0)
                && (y_barriers_bottom.len() % 2 == 0)
                && (y_barriers_top.len() % 2 == 0));

            if is_inside {
                points_inside.insert(*point);
                count += 1;
                continue;
            }
        }
        // self.print_subset(&loop_members, &points_inside, true);
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
    fn test_part_2_complex() {
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
