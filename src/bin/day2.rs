use std::fs;

fn part1(input: &str) -> u32 {
    let games = parse_input(input);

    games
        .iter()
        .filter(|game| {
            game.is_possible(Cubes {
                red: 12,
                green: 13,
                blue: 14,
            })
        })
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    let games = parse_input(input);

    games
        .iter()
        .map(|game| game.min_set_of_cubes())
        .map(|cubes| cubes.power())
        .sum()
}

fn main() {
    let input = fs::read_to_string("src/bin/input2.txt").unwrap();

    println!("Answer to day2 part 1: {}", part1(&input));
    println!("Answer to day2 part 2: {}", part2(&input));
}

fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(Game::parse).collect()
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    cubes_revealed: Vec<Cubes>,
}

impl Game {
    pub fn parse(input: &str) -> Game {
        let mut parts = input.split(":");
        let id = parts
            .next()
            .unwrap()
            .strip_prefix("Game ")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let cubes_revealed = parts.next().unwrap().split(";").map(Cubes::parse).collect();

        Game { id, cubes_revealed }
    }

    pub fn is_possible(&self, cubes: Cubes) -> bool {
        self.cubes_revealed
            .iter()
            .all(|cr| cr.red <= cubes.red && cr.green <= cubes.green && cr.blue <= cubes.blue)
    }

    pub fn min_set_of_cubes(&self) -> Cubes {
        Cubes {
            blue: self.cubes_revealed.iter().map(|cr| cr.blue).max().unwrap(),
            green: self.cubes_revealed.iter().map(|cr| cr.green).max().unwrap(),
            red: self.cubes_revealed.iter().map(|cr| cr.red).max().unwrap(),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
struct Cubes {
    blue: u32,
    green: u32,
    red: u32,
}

impl Cubes {
    pub fn parse(input: &str) -> Cubes {
        let mut result = Cubes::default();
        for extraction in input.split(",") {
            let mut parts = extraction.trim().splitn(2, " ");
            let count: u32 = parts.next().unwrap().trim().parse().unwrap();
            let color = parts.next().unwrap().trim();
            match color {
                "blue" => result.blue += count,
                "red" => result.red += count,
                "green" => result.green += count,
                _ => panic!(),
            }
        }
        result
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(
            Game {
                id: 1,
                cubes_revealed: vec![
                    Cubes {
                        blue: 3,
                        red: 4,
                        green: 0
                    },
                    Cubes {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Cubes {
                        green: 2,
                        red: 0,
                        blue: 0
                    }
                ]
            },
            Game::parse(input)
        );
    }

    #[test]
    fn test_part_1() {
        let example_input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(8, part1(example_input));
    }

    #[test]
    fn test_part_2() {
        let example_input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(2286, part2(example_input));
    }
}
