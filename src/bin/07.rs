use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(7);

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    Unknown,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [u32; 5],
    bid: u32,
}

fn get_hand_type_with_jokers(hand: [u32; 5]) -> HandType {
    if !hand.contains(&1) {
        return get_hand_type(hand);
    }

    let joker_count = hand.iter().filter(|&&c| c == 1).count();

    if joker_count == 4 || joker_count == 5 {
        return HandType::FiveOfAKind;
    }

    let non_jokers = hand.iter().filter(|&&c| c != 1).collect_vec();
    let mut occurrences = HashMap::new();

    for &value in non_jokers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    let mode = occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .unwrap();

    get_hand_type(hand.map(|v| if v == 1 { mode } else { v }))
}

fn get_hand_type(hand: [u32; 5]) -> HandType {
    let sorted_cards = itertools::sorted(hand).collect::<Vec<_>>();

    let matching_cards = sorted_cards
        .iter()
        .group_by(|&x| x)
        .into_iter()
        .map(|(_, group)| group.count())
        .collect::<Vec<_>>();

    match matching_cards.iter().max() {
        Some(1) => HandType::HighCard,
        Some(2) if matching_cards.iter().filter(|&&c| c == 2).count() == 2 => HandType::TwoPair,
        Some(2) => HandType::OnePair,
        Some(3) if matching_cards.contains(&2) => HandType::FullHouse,
        Some(3) => HandType::ThreeOfAKind,
        Some(4) => HandType::FourOfAKind,
        Some(5) => HandType::FiveOfAKind,
        _ => HandType::Unknown,
    }
}

fn card_value(card: char, part_two: bool) -> u32 {
    match card {
        x if x.is_ascii_digit() => x.to_digit(10).unwrap(),
        'T' => 10,
        'J' if part_two => 1,
        'J' if !part_two => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

fn parse_hand(hand: &str, part_two: bool) -> Hand {
    let (cards, bid) = hand.split_once(' ').unwrap();

    let card_values: [u32; 5] = cards
        .chars()
        .map(|c| card_value(c, part_two))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let hand_type = if part_two {
        get_hand_type_with_jokers(card_values)
    } else {
        get_hand_type(card_values)
    };

    Hand {
        cards: card_values,
        bid: bid.parse().unwrap(),
        hand_type,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| parse_hand(line, false))
            .sorted()
            .enumerate()
            .map(|(i, hand)| (i as u32 + 1) * hand.bid)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let sorted_hands = input
        .lines()
        .map(|line| parse_hand(line, true))
        .sorted()
        .collect::<Vec<_>>();

    Some(
        sorted_hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i as u32 + 1) * hand.bid)
            .sum::<u32>(),
    )
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
