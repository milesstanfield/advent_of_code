use std::{cmp::Ordering, collections::HashMap};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FIVE = 7,
    FOUR = 6,
    FULL = 5,
    THREE = 4,
    TWOP = 3,
    ONEP = 2,
    HIGH = 1,
}

#[derive(Debug, Eq, PartialOrd)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    hand_type: HandType,
}

struct SizedCards {
    cards: Vec<i32>,
}
impl SizedCards {
    fn new(source_cards: &Vec<char>) -> Self {
        let card_map = Self::card_map();
        let cards: Vec<i32> = source_cards.into_iter().map(|c| card_map[&c]).collect();
        SizedCards { cards }
    }

    fn card_map() -> HashMap<char, i32> {
        HashMap::from([
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
            ('J', 1),
        ])
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type && self.cards != other.cards {
            let sized_cards1 = SizedCards::new(&self.cards).cards;
            let sized_cards2 = SizedCards::new(&other.cards).cards;
            return sized_cards1.cmp(&sized_cards2);
        }
        self.hand_type.cmp(&other.hand_type)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl From<&str> for Hand {
    fn from(line: &str) -> Self {
        let re = Regex::new(r"(.*) (\d+)").unwrap();
        let mut cards: Vec<char> = vec![];
        let mut bid: usize = 0;

        for (_, [cards_str, bid_str]) in re.captures_iter(line).map(|x| x.extract()) {
            cards = cards_str.chars().collect();
            bid = bid_str.parse().expect("NaN");
        }

        let hand_type = Hand::hand_type(&cards);
        return Hand {
            cards,
            bid,
            hand_type,
        };
    }
}

impl Hand {
    fn hand_type(cards: &Vec<char>) -> HandType {
        let map = Self::card_counts_map(cards);
        let counts: Vec<&usize> = map.values().collect();

        if counts.contains(&&5) {
            HandType::FIVE
        } else if counts.contains(&&4) {
            HandType::FOUR
        } else if counts.contains(&&3) && counts.contains(&&2) {
            HandType::FULL
        } else if counts.contains(&&3) {
            HandType::THREE
        } else {
            let pairs: Vec<&usize> = counts.into_iter().filter(|c| c == &&2).collect();
            if pairs.len() == 2 {
                HandType::TWOP
            } else if pairs.len() == 1 {
                HandType::ONEP
            } else {
                HandType::HIGH
            }
        }
    }

    fn card_counts_map(cards: &Vec<char>) -> HashMap<char, usize> {
        let mut map: HashMap<char, usize> = HashMap::new();
        for card in cards {
            if let Some(val) = map.get(card) {
                map.insert(*card, val + 1);
            } else {
                map.insert(*card, 1);
            }
        }
        map
    }
}

pub fn run(input: &String) -> usize {
    let mut sum: usize = 0;
    let mut hands: Vec<Hand> = input.lines().map(|line| Hand::from(line)).collect();
    hands.sort_by(|a, b| a.cmp(b));

    for (i, hand) in hands.into_iter().enumerate() {
        sum += hand.bid * (i + 1);
    }

    dbg!(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: String = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .into();
        assert_eq!(run(&input), 5905);
    }
}
