use itertools::Itertools;

advent_of_code::solution!(9);

fn get_difference(seq: &[i32]) -> Vec<i32> {
    seq.iter()
        .zip(seq.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect_vec()
}

fn next_term(seq: &[i32]) -> i32 {
    let diff = get_difference(seq);

    if diff.iter().all(|&n| n == 0) {
        return *seq.last().unwrap();
    }

    return seq.last().unwrap() + next_term(&diff);
}

fn prev_term(seq: &[i32]) -> i32 {
    let diff = get_difference(seq);

    if diff.iter().all(|&n| n == 0) {
        return *seq.first().unwrap();
    }

    return seq.first().unwrap() - prev_term(&diff);
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect_vec()
            })
            .map(|seq| next_term(seq.as_slice()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect_vec()
            })
            .map(|seq| prev_term(seq.as_slice()))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
