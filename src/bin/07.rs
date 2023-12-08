use std::char;
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::Rank::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Rank {
    FiveOfAKind(u8),
    FourOfAKind(u8),
    FullHouse(u8),
    ThreeOfAKind(u8),
    TwoPair(u8),
    OnePair(u8),
    HighCard(u8),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Hand {
    cards: Vec<char>,
    rank: Rank,
}

impl Hand {
    fn new(cards: Vec<char>, rank: Rank) -> Self {
        Hand { cards, rank }
    }
}

type Hands = (Hand, usize);

fn rank(cards: Vec<char>) -> Rank {
    let mut freq_map: HashMap<char, usize> = HashMap::new();
    for &card in &cards {
        *freq_map.entry(card).or_insert(0) += 1;
    }
    let max_freq = *freq_map.values().max().unwrap();

    match max_freq {
        5 => FiveOfAKind(0),
        4 => FourOfAKind(1),
        3 => {
            if freq_map.len() == 2 {
                FullHouse(2)
            } else {
                ThreeOfAKind(3)
            }
        }
        2 => {
            let pairs = freq_map.iter().filter(|&(_, &freq)| freq == 2).count();
            if pairs == 2 {
                TwoPair(4)
            } else {
                OnePair(5)
            }
        }
        _ => HighCard(6),
    }
}

fn rank_part_two(cards: Vec<char>) -> Rank {
    let current_rank = rank(cards.clone());
    let mut freq_map: HashMap<char, usize> = HashMap::new();
    for &card in &cards {
        *freq_map.entry(card).or_insert(0) += 1;
    }
    let freq_j = freq_map.entry('J').or_default();
    if *freq_j == 0 {
        return current_rank;
    }
    match current_rank {
        FiveOfAKind(0) => FiveOfAKind(0),
        FourOfAKind(1) => FiveOfAKind(0),
        FullHouse(2) => FiveOfAKind(0),
        ThreeOfAKind(3) => FourOfAKind(1),
        TwoPair(4) => {
            if *freq_j == 2 {
                FourOfAKind(1)
            } else {
                FullHouse(2)
            }
        }
        OnePair(5) => ThreeOfAKind(3),
        HighCard(6) => OnePair(5),
        _ => panic!("Invalid current rank: {:?}", current_rank),
    }
}

fn compare_cards(a: char, b: char, cards_order: &str) -> Ordering {
    let rank_a = cards_order.find(a).unwrap();
    let rank_b = cards_order.find(b).unwrap();

    rank_a.cmp(&rank_b)
}

fn cmp_hands(a: &Hand, b: &Hand, cards_ordering: &str) -> Ordering {
    match a.rank.cmp(&b.rank) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            let ordering: Vec<Ordering> = a
                .cards
                .iter()
                .zip(b.cards.iter())
                .skip_while(|hand| hand.0 == hand.1)
                .map(|hand| compare_cards(*hand.0, *hand.1, cards_ordering))
                .collect();
            *ordering.first().unwrap()
        }
    }
}

fn parse_input(input: &str, ranking_fn: fn(Vec<char>) -> Rank) -> Vec<Hands> {
    let hands: Vec<Hands> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let cards: Vec<char> = parts[0].chars().collect();
            let hand = Hand::new(cards.clone(), ranking_fn(cards));
            let bidding = parts[1].parse::<usize>().unwrap();
            (hand, bidding)
        })
        .collect();
    hands
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut hands = parse_input(input, rank);

    // Sorting hands based on rank
    hands.sort_by(|a, b| cmp_hands(&a.0, &b.0, "AKQJT98765432"));

    let mut total_winnings: usize = 0;
    for (idx, hand) in hands.iter().rev().enumerate() {
        //println!("{:?} - {:?} - {}", hand.0.rank, hand.0.cards, hand.1);
        total_winnings += hand.1 * (idx + 1);
    }
    Some(total_winnings)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut hands: Vec<Hands> = parse_input(input, rank_part_two);

    // Sorting hands based on rank
    hands.sort_by(|a, b| cmp_hands(&a.0, &b.0, "AKQT98765432J"));

    // Display sorted hands
    let mut total_winnings: usize = 0;
    for (idx, hand) in hands.iter().rev().enumerate() {
        //println!("{:?} - {:?} - {}", hand.0.rank, hand.0.cards, hand.1);
        total_winnings += hand.1 * (idx + 1);
    }
    Some(total_winnings)
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
