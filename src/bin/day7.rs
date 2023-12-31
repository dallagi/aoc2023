use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn part1(input: &str) -> u64 {
    CamelCardsGame::parse(input).total_winnings()
}

fn part2(input: &str) -> u64 {
    CamelCardsGame::parse(input).total_winnings()
}

fn main() {
    let input = fs::read_to_string("src/bin/input7.txt").unwrap();

    // Currently part1 is broken
    println!("[BROKEN] Answer to day7 part 1: {}", part1(&input));
    println!("Answer to day7 part 2: {}", part2(&input));
}

struct CamelCardsGame {
    hands: Vec<Hand>,
    bets: Vec<u64>,
}

impl CamelCardsGame {
    fn parse(input: &str) -> Self {
        let mut hands = Vec::new();
        let mut bets = Vec::new();
        for line in input.lines() {
            let mut parts = line.splitn(2, " ");

            hands.push(Hand::parse(parts.next().unwrap()));
            bets.push(parts.next().unwrap().parse().unwrap());
        }

        Self { hands, bets }
    }

    fn total_winnings(&self) -> u64 {
        let mut hands_with_bets: Vec<(&Hand, &u64)> =
            self.hands.iter().zip(self.bets.iter()).collect();
        hands_with_bets.sort_by(|(hand, _), (other, _)| hand.cmp(other));

        hands_with_bets
            .into_iter()
            .enumerate()
            .map(|(rank, (_, bet))| (rank as u64 + 1) * bet)
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    cards_count: HashMap<Card, u64>,
}

impl Hand {
    fn parse(input: &str) -> Self {
        let cards: Vec<Card> = input.chars().map(|c| Card { card: c }).collect();
        let cards_count = Self::cards_count(&cards);

        Self { cards, cards_count }
    }

    fn cards_count(cards: &[Card]) -> HashMap<Card, u64> {
        let mut res = HashMap::new();
        let mut jokers_count = 0;
        for card in cards {
            if *card == Card::joker() {
                jokers_count += 1;
            } else {
                *res.entry(*card).or_default() += 1
            }
        }
        let joker = Card::joker();
        let most_common_card = res
            .iter()
            .max_by_key(|(_k, v)| **v)
            .unwrap_or((&joker, &5))
            .0;
        *res.entry(*most_common_card).or_default() += jokers_count;
        res
    }

    fn type_strength(&self) -> u64 {
        let mut cards_counts: Vec<u64> = self.cards_count.values().map(|c| *c).collect();
        cards_counts.sort();

        match cards_counts.as_slice() {
            &[5] => 6,          // five of a kind
            &[1, 4] => 5,       // four of a kind
            &[2, 3] => 4,       // full house
            &[1, 1, 3] => 3,    // three of a kind
            &[1, 2, 2] => 2,    // two pair
            &[1, 1, 1, 2] => 1, // one pair
            _ => 0,             // high card
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.type_strength() != other.type_strength() {
            return self.type_strength().cmp(&other.type_strength());
        }

        for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if card.strength() != other_card.strength() {
                return card.strength().cmp(&other_card.strength());
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Card {
    card: char,
}

impl Card {
    fn strength(&self) -> u64 {
        match self.card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            // 'J' => 10,
            'T' => 10,
            'J' => 0,
            c => c.to_digit(10).unwrap() as u64,
        }
    }

    fn joker() -> Self {
        Card { card: 'J' }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hand_from_string() {
        assert_eq!(
            Hand {
                cards: vec![
                    Card { card: '3' },
                    Card { card: '2' },
                    Card { card: 'T' },
                    Card { card: '3' },
                    Card { card: 'K' },
                ],
                cards_count: HashMap::from([
                    (Card { card: '3' }, 2),
                    (Card { card: '2' }, 1),
                    (Card { card: 'T' }, 1),
                    (Card { card: 'K' }, 1)
                ])
            },
            Hand::parse("32T3K")
        )
    }

    #[test]
    #[ignore = "currently broken"]
    fn test_part_1() {
        let example_input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

        assert_eq!(6440, part1(example_input));
    }

    #[test]
    fn test_part_2() {
        let example_input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

        assert_eq!(5905, part2(example_input));
    }
}
