use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Ord, Eq)]
struct Hand {
    cards: Vec<u8>,
    hand_type: HandType,
    bid: usize,
    p2: bool,
}

fn parse_hand(value: &str, p2: bool) -> Hand {
    let (cards, bid) = value.split_once(" ").unwrap();
    let cards = parse_cards(cards, p2);
    let ht = hand_type(&cards, p2);
    Hand { cards, hand_type: ht, bid: bid.parse::<usize>().unwrap(), p2 }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ht_cmp = self.hand_type.cmp(&other.hand_type);
        Some(match ht_cmp {
            Ordering::Equal => self.cards.cmp(&other.cards),
            _ => ht_cmp,
        })
    }
}

fn parse_cards(hand: &str, p2: bool) -> Vec<u8> {
    hand.chars()
        .map(|c| match c {
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => if !p2 { 11 } else { 0 },
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Unknown card")
        })
        .collect_vec()
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    High,
    Pair,
    TwoPairs,
    Three,
    House,
    Four,
    Five,
}

fn hand_type(value: &Vec<u8>, p2: bool) -> HandType {
    let mut groups: HashMap<u8, Vec<u8>> = HashMap::new();
    value.iter()
        .for_each(|n| {
            if *n > 0 { groups.entry(*n).or_insert_with(Vec::new).push(*n) }
        });
    match groups.values().map(|v| v.iter().count()).sorted().as_slice() {
        &[1, 1, 1, 1, 1] => HandType::High,
        &[1, 1, 1, 2] => HandType::Pair,
        &[1, 2, 2] => HandType::TwoPairs,
        &[1, 1, 3] => HandType::Three,
        &[2, 3] => HandType::House,
        &[1, 4] => HandType::Four,
        &[5] => HandType::Five,
        _ if !p2 => panic!("Unknown hand {:?}", value),

        // 1 J
        &[4] => HandType::Five,
        &[1, 3] => HandType::Four,
        &[2, 2] => HandType::House,
        &[1, 1, 2] => HandType::Three,
        &[1, 1, 1, 1] => HandType::Pair,
        // 2 J
        &[3] => HandType::Five,
        &[1, 2] => HandType::Four,
        &[1, 1, 1] => HandType::Three,
        // 3 J
        &[2] => HandType::Five,
        &[1, 1] => HandType::Four,
        // 4 J
        &[1] => HandType::Five,
        // 5 J
        &[] => HandType::Five,
        _ => panic!("Unknown hand {:?}", value)
    }
}

fn get_hands(input: &str, p2: bool) -> Vec<Hand> {
    input.lines().map(|l| parse_hand(l, p2)).collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let hands = get_hands(input, false);
    hands.iter().sorted().enumerate().map(|(i, h)| (i + 1) * h.bid).sum1()
}

pub fn part_two(input: &str) -> Option<usize> {
    let hands = get_hands(input, true);
    hands.iter().sorted().enumerate().map(|(i, h)| (i + 1) * h.bid).sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
