use std::collections::HashSet;
use std::fs;

fn part1(input: &str) -> u32 {
    Cards::parse(input)
        .cards
        .iter()
        .map(Card::winning_numbers_count)
        .filter(|count| *count > 0)
        .map(|count| 2_u32.pow(count as u32 - 1))
        .sum()
}

fn part2(input: &str) -> u32 {
    let cards = Cards::parse(input);
    cards.sum_of_winning_card_instances(0, cards.cards.len() - 1) - 1
}

fn main() {
    let input = fs::read_to_string("src/bin/input4.txt").unwrap();

    println!("Answer to day4 part 1: {}", part1(&input));
    println!("Answer to day4 part 2: {}", part2(&input));
}

struct Cards {
    pub cards: Vec<Card>,
}

impl Cards {
    fn parse(input: &str) -> Self {
        Self {
            cards: input.lines().map(Card::parse).collect(),
        }
    }

    fn sum_of_winning_card_instances(&self, from_index: usize, to_index: usize) -> u32 {
        let mut res = 0;
        for i in from_index..=to_index {
            let card = self.cards.get(i).unwrap();
            let cards_won_count = card.winning_numbers_count();
            res += self.sum_of_winning_card_instances(i + 1, i + cards_won_count);
        }
        1 + res
    }
}

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
    winning_numbers_count: usize,
}

impl Card {
    fn new(id: u32, winning: HashSet<u32>, numbers: HashSet<u32>) -> Self {
        let winning_numbers_count = winning.intersection(&numbers).count();
        Self {
            id,
            winning,
            numbers,
            winning_numbers_count,
        }
    }

    fn parse(input: &str) -> Self {
        let mut parts = input.splitn(3, [':', '|']);
        let id = parts
            .next()
            .unwrap()
            .strip_prefix("Card ")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let winning = parts
            .next()
            .unwrap()
            .split(" ")
            .filter(|s| !s.trim().is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        let numbers = parts
            .next()
            .unwrap()
            .split(" ")
            .filter(|s| !s.trim().is_empty())
            .map(|n| n.parse().unwrap())
            .collect();

        Self::new(id, winning, numbers)
    }

    fn winning_numbers_count(&self) -> usize {
        self.winning_numbers_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let card_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        assert_eq!(
            Card {
                id: 1,
                winning: HashSet::from_iter(vec![41, 48, 83, 86, 17]),
                numbers: HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53]),
                winning_numbers_count: 4
            },
            Card::parse(card_input)
        );
    }

    #[test]
    fn test_part_1() {
        let example_input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        assert_eq!(13, part1(example_input));
    }

    #[test]
    fn test_part_2() {
        let example_input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        assert_eq!(30, part2(example_input));
    }
}
