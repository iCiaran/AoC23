advent_of_code::solution!(1);

fn get_digits(line: &str) -> Vec<u32> {
    line.chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn get_digits_with_words(line: &str) -> Vec<u32> {
    let number_map: Vec<(&str, u32)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut digits: Vec<(usize, u32)> = line
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_ascii_digit())
        .map(|(i, c)| (i, c.to_digit(10).unwrap()))
        .collect();

    let mut word_digits: Vec<(usize, u32)> = number_map
        .iter()
        .flat_map(|(k, v)| {
            line.match_indices(k)
                .map(move |(i, _)| (i, *v))
                .collect::<Vec<_>>()
        })
        .collect();

    digits.append(&mut word_digits);
    digits.sort_by(|(i, _), (j, _)| i.cmp(j));

    digits.into_iter().map(|(_, b)| b).collect()
}

fn get_calibration(digits: Vec<u32>) -> u32 {
    assert!(!digits.is_empty(), "no digits found");

    let first = digits.first().unwrap();
    let last = digits.last().unwrap();

    first * 10 + last
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(get_digits).map(get_calibration).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(get_digits_with_words)
            .map(get_calibration)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, "a",
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, "b",
        ));
        assert_eq!(result, Some(281));
    }
}
