use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

use itertools::FoldWhile::{Continue, Done};

fn parse_input(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap();
    lines.next();

    let maps = lines
        .map(|line| line.split_once(" = ").unwrap())
        .map(|(node, directions)| {
            (
                node,
                directions
                    .split_once(", ")
                    .map(|(l, r)| (&l[1..], &r[..3]))
                    .unwrap(),
            )
        });

    (instructions, HashMap::from_iter(maps))
}

fn steps_to_node(
    instructions: &str,
    maps: &HashMap<&str, (&str, &str)>,
    start: &str,
    found: fn(&str) -> bool,
) -> u32 {
    let (_, steps) = instructions
        .chars()
        .cycle()
        .fold_while((start, 0), |(node, steps), dir| {
            if found(node) {
                Done((&node, steps))
            } else {
                let (l, r) = maps.get(node).unwrap();
                Continue((if dir == 'L' { &l } else { &r }, steps + 1))
            }
        })
        .into_inner();
    steps
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);

    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, maps) = parse_input(input);

    Some(steps_to_node(instructions, &maps, "AAA", |s| s == "ZZZ"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, maps) = parse_input(input);

    let starting_nodes = maps.keys().filter(|key| key.ends_with('A')).collect_vec();

    let cycles = starting_nodes
        .iter()
        .map(|start| steps_to_node(instructions, &maps, start, |s| s.ends_with('Z')))
        .collect_vec();

    Some(lcm(&cycles.iter().map(|&c| c as u64).collect_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, "a",
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, "b",
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, "c",
        ));
        assert_eq!(result, Some(6));
    }
}
