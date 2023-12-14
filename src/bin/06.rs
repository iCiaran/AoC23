advent_of_code::solution!(6);

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let lines = input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(_, line)| line)
        .map(str::split_whitespace)
        .map(|line| {
            line.map(|value| value.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>();

    lines[0]
        .iter()
        .cloned()
        .zip(lines[1].iter().cloned())
        .collect::<Vec<_>>()
}

fn number_of_winning_times(time: &u64, distance: &u64) -> u64 {
    let t = *time as f64;
    let d = *distance as f64;

    let disc = t * t - (4.0 * d);

    assert!(disc >= 0.0);

    let root = disc.sqrt();

    let x = (t - root) / 2.0;
    let y = (t + root) / 2.0;

    let min = if x.fract() == 0.0 {
        (x as u64) + 1
    } else {
        x.ceil() as u64
    };

    let max = if y.fract() == 0.0 {
        (y as u64) - 1
    } else {
        y.floor() as u64
    };

    max - min + 1
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .map(|(time, distance)| number_of_winning_times(time, distance))
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_without_whitespace = input
        .chars()
        .filter(|&c| !c.is_whitespace() || c == '\n')
        .collect::<String>();

    Some(
        parse_input(input_without_whitespace.as_str())
            .iter()
            .map(|(time, distance)| number_of_winning_times(time, distance))
            .product(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
