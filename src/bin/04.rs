advent_of_code::solution!(4);

use std::collections::HashMap;

fn parse_numbers(numbers: &str) -> Vec<u32> {
    numbers
        .split_whitespace()
        .map(|n| n.trim())
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    line.split_once(':')
        .map(|(_, numbers)| numbers)
        .map(|numbers| numbers.split_once('|').unwrap())
        .map(|(winning, mine)| (parse_numbers(winning), parse_numbers(mine)))
        .unwrap()
}

fn count_winning_numbers(mine: &[u32], winning: &[u32]) -> u32 {
    mine.iter()
        .filter(|&number| winning.contains(number))
        .count() as u32
}

// fn count_cards(
//     card: &u32,
//     max_card: &u32,
//     cards: &HashMap<u32, Vec<u32>>,
//     mut seen: &HashMap<u32, u32>,
// ) -> u32 {
//     if seen.contains_key(&card) {
//         return *seen.get(&card).unwrap();
//     }

//     let next_cards = cards
//         .get(card)
//         .unwrap()
//         .clone()
//         .iter()
//         .filter(|&c| c <= max_card)
//         .collect::<u32>();

//     if next_cards.is_empty() {
//         return 0;
//     }

//     1
// }

fn count_cards(card: u32, cards: &[u32], seen: &mut HashMap<u32, u32>) -> u32 {
    if let Some(exists) = seen.get(&card) {
        return *exists;
    }

    let next_cards = ((card + 1)..(card + 1 + cards[(card - 1) as usize]))
        .filter(|&c| c <= cards.len() as u32)
        .collect::<Vec<_>>();

    if next_cards.is_empty() {
        return 0;
    }

    let total = next_cards
        .into_iter()
        .map(|next| 1 + count_cards(next, cards, seen))
        .sum::<u32>();

    seen.insert(card, total);

    total
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_line)
            .map(|(winning, mine)| count_winning_numbers(&mine, &winning))
            .map(|count| {
                if count == 0 {
                    0
                } else {
                    u32::pow(2, count - 1)
                }
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input
        .lines()
        .map(parse_line)
        .map(|(winning, mine)| count_winning_numbers(&mine, &winning))
        .collect::<Vec<_>>();

    let mut card_count = HashMap::<u32, u32>::new();

    let total = (1..=cards.len())
        .map(|card| count_cards(card as u32, &cards, &mut card_count))
        .sum::<u32>();

    Some(cards.len() as u32 + total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
