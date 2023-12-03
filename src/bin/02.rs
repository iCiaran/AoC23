advent_of_code::solution!(2);

#[derive(Default)]
struct Colours {
    red: u32,
    green: u32,
    blue: u32,
}

fn game_is_possible(game_colours: &Colours, real_colours: &Colours) -> bool {
    real_colours.red >= game_colours.red
        && real_colours.green >= game_colours.green
        && real_colours.blue >= game_colours.blue
}

fn parse_line(line: &str) -> (u32, Vec<Colours>) {
    line.split_once(':')
        .map(|(number, colours)| (parse_game_number(number), parse_all_colours(colours)))
        .unwrap()
}

fn parse_game_number(number: &str) -> u32 {
    number
        .strip_prefix("Game ")
        .map(|number| number.parse::<u32>())
        .unwrap()
        .unwrap()
}

fn parse_all_colours(all_colours: &str) -> Vec<Colours> {
    all_colours.split(';').map(parse_colours).collect()
}

fn parse_colours(colours: &str) -> Colours {
    colours.split(',').fold(Colours::default(), |acc, c| {
        let (number, colour) = parse_colour(c);
        match colour {
            "red" => Colours { red: number, ..acc },
            "green" => Colours { green: number, ..acc },
            "blue" => Colours { blue: number, ..acc },
            _ => acc,
        }
    })
}

fn parse_colour(colour: &str) -> (u32, &str) {
    colour
        .trim()
        .split_once(' ')
        .map(|(number, colour)| (number.parse::<u32>().unwrap(), colour))
        .unwrap()
}

fn max_colours(colours: Vec<Colours>) -> Colours {
    colours.iter().fold(Colours::default(), |acc, c| Colours {
        red: std::cmp::max(acc.red, c.red),
        green: std::cmp::max(acc.green, c.green),
        blue: std::cmp::max(acc.blue, c.blue),
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let real_colours = Colours { red: 12, green: 13, blue: 14 };

    Some(
        input
            .lines()
            .map(parse_line)
            .map(|(number, colours)| (number, max_colours(colours)))
            .filter(|(_, colours)| game_is_possible(colours, &real_colours))
            .map(|(number, _)| number)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_line)
            .map(|(_, colours)| max_colours(colours))
            .map(|min_colours| min_colours.red * min_colours.green * min_colours.blue)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
