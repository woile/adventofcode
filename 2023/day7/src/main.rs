use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq)]
enum Hand<'a> {
    FiveOfAKind(&'a str),
    FourOfAKind(&'a str),
    FullHouse(&'a str),
    ThreeOfAKind(&'a str),
    TwoPair(&'a str),
    OnePair(&'a str),
    HighCard(&'a str),
}

impl Hand<'_> {
    fn cmp_internal(this: &str, other: &str) -> Ordering {
        if this == other {
            return Ordering::Equal;
        }

        let mut self_cards = this.chars();
        let mut other_cards = other.chars();

        loop {
            let self_card = self_cards.next();
            let other_card = other_cards.next();
            match (self_card, other_card) {
                (Some(self_card), Some(other_card)) => {
                    let self_card = match self_card {
                        'A' => 'T',
                        'J' => 'J',
                        'K' => 'Q',
                        'Q' => 'K',
                        'T' => 'A',
                        _ => self_card,
                    };
                    let other_card = match other_card {
                        'A' => 'T',
                        'J' => 'J',
                        'K' => 'Q',
                        'Q' => 'K',
                        'T' => 'A',
                        _ => other_card,
                    };
                    match self_card.cmp(&other_card) {
                        Ordering::Equal => continue,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                    }
                }
                (Some(_), None) => return Ordering::Greater,
                (None, Some(_)) => return Ordering::Less,
                (None, None) => return Ordering::Equal,
            }
        }
    }
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(cards: &'a str) -> Self {
        let mut _cards = cards.chars();
        let mut card_counts = HashMap::new();
        for card in _cards {
            let count = card_counts.entry(card).or_insert(0);
            *count += 1;
        }
        let mut counts = card_counts.values().collect::<Vec<_>>();
        counts.sort();
        counts.reverse();
        match counts.as_slice() {
            [1, 1, 1, 1, 1] => Hand::HighCard(cards),
            [2, 1, 1, 1] => Hand::OnePair(cards),
            [2, 2, 1] => Hand::TwoPair(cards),
            [3, 1, 1] => Hand::ThreeOfAKind(cards),
            [3, 2] => Hand::FullHouse(cards),
            [4, 1] => Hand::FourOfAKind(cards),
            [5] => Hand::FiveOfAKind(cards),
            _ => panic!("Invalid hand"),
        }
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Hand::FiveOfAKind(_this), Hand::FiveOfAKind(_other)) => {
                Hand::cmp_internal(_this, _other)
            }
            (Hand::FourOfAKind(_this), Hand::FourOfAKind(_other)) => {
                Hand::cmp_internal(_this, _other)
            }
            (Hand::FullHouse(_this), Hand::FullHouse(_other)) => Hand::cmp_internal(_this, _other),
            (Hand::TwoPair(_this), Hand::TwoPair(_other)) => Hand::cmp_internal(_this, _other),
            (Hand::ThreeOfAKind(_this), Hand::ThreeOfAKind(_other)) => {
                Hand::cmp_internal(_this, _other)
            }
            (Hand::OnePair(_this), Hand::OnePair(_other)) => Hand::cmp_internal(_this, _other),
            (Hand::HighCard(_this), Hand::HighCard(_other)) => Hand::cmp_internal(_this, _other),
            (Hand::FiveOfAKind(_), _) => Ordering::Greater,
            (_, Hand::FiveOfAKind(_)) => Ordering::Less,
            (Hand::FourOfAKind(_), _) => Ordering::Greater,
            (_, Hand::FourOfAKind(_)) => Ordering::Less,
            (Hand::FullHouse(_), _) => Ordering::Greater,
            (_, Hand::FullHouse(_)) => Ordering::Less,
            (Hand::ThreeOfAKind(_), _) => Ordering::Greater,
            (_, Hand::ThreeOfAKind(_)) => Ordering::Less,
            (Hand::TwoPair(_), _) => Ordering::Greater,
            (_, Hand::TwoPair(_)) => Ordering::Less,
            (Hand::OnePair(_), _) => Ordering::Greater,
            (_, Hand::OnePair(_)) => Ordering::Less,
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Hand::FiveOfAKind(hand1), Hand::FiveOfAKind(hand2)) => hand1 == hand2,
            (Hand::FourOfAKind(hand1), Hand::FourOfAKind(hand2)) => hand1 == hand2,
            (Hand::FullHouse(hand1), Hand::FullHouse(hand2)) => hand1 == hand2,
            (Hand::ThreeOfAKind(hand1), Hand::ThreeOfAKind(hand2)) => hand1 == hand2,
            (Hand::TwoPair(hand1), Hand::TwoPair(hand2)) => hand1 == hand2,
            (Hand::OnePair(hand1), Hand::OnePair(hand2)) => hand1 == hand2,
            (Hand::HighCard(hand1), Hand::HighCard(hand2)) => hand1 == hand2,
            _ => false,
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut hands = input.lines().map(|line| line.split_at(5)).map(|(hand, bid)| {
        (Hand::from(hand), bid.trim().parse::<u64>().expect(format!("Invalid bid: {}", bid).as_str()))
    }).collect::<Vec<(Hand, u64)>>();
    hands.sort_by(|(h1,_), (h2,_)| h1.cmp(h2));
    hands.iter().enumerate().map(|(i, (_, bid))| bid * (i as u64 + 1)).sum()
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("{}", part1(raw.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chars_cmp() {
        assert!('A' > '2');
        assert!('A' > '9');
        assert!('T' > '9');
        assert!('T' > '9');
        assert!('9' > '2');
        assert!('3' > '2');

    }

    #[test]
    fn test_hand_partial_eq() {
        assert!(Hand::FiveOfAKind("KK677") == Hand::FiveOfAKind("KK677"));
        assert!(Hand::FiveOfAKind("KK677") != Hand::FiveOfAKind("JK677"));
        assert!(Hand::FiveOfAKind("KK677") != Hand::TwoPair("JK677"));
    }

    #[test]
    fn test_hand_cmp_internal() {
        Hand::cmp_internal("33332", "2AAAA");
        assert!(Hand::cmp_internal("33332", "2AAAA") == Ordering::Greater);
        assert!(Hand::cmp_internal("77888", "77788") == Ordering::Greater);
        assert!(Hand::cmp_internal("AA888", "TTT88") == Ordering::Greater);
    }

    #[test]
    fn test_cmp() {
        assert!(Hand::from("33332") > Hand::from("2AAAA"));
        assert!(Hand::from("77888") > Hand::from("77788"));
        assert!(Hand::from("AA888") > Hand::from("TTT88"));
        assert!(Hand::from("AA888") > Hand::from("KKK88"));
        assert!(Hand::from("AA888") > Hand::from("QQQ88"));
        assert!(Hand::from("AA888") > Hand::from("JJJ88"));
        assert!(Hand::from("AA888") > Hand::from("99988"));
        assert!(Hand::from("KK888") > Hand::from("QQQ88"));
        assert!(Hand::from("KK888") > Hand::from("JJJ88"));
        assert!(Hand::from("KK888") > Hand::from("99988"));
    }
    #[test]
    fn test_from_str() {
        let input = "AAAAA";
        let hand = Hand::from(input);
        assert_eq!(hand, Hand::FiveOfAKind(input));

        let input = "AA8AA";
        let hand = Hand::from(input);
        assert_eq!(hand, Hand::FourOfAKind(input));

        let input = "23332";
        let hand = Hand::from(input);
        assert_eq!(hand, Hand::FullHouse(input));

        let input = "TTT98";
        let hand = Hand::from(input);
        assert_eq!(hand, Hand::ThreeOfAKind(input));

        let input = "23432";
        let hand = Hand::from(input);
        assert_eq!(hand, Hand::TwoPair(input));

        let input = "A23A4";
        let hand = Hand::from(input);
        assert_eq!(hand, Hand::OnePair(input));

        let input = "23456";
        let hand = Hand::from(input);
        assert_eq!(hand, Hand::HighCard(input));
    }

    #[test]
    fn test_split_at() {
        let input = "AAAAA 1";
        let (hand, bid) = input.split_at(5);
        assert_eq!(hand, "AAAAA");
        assert_eq!(bid, " 1");
    }
}
