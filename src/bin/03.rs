advent_of_code::solution!(3);

struct Number {
    row: i32,
    column: i32,
    length: i32,
    value: u32,
}

struct Symbol {
    row: i32,
    column: i32,
    maybe_gear: bool,
}

struct Engine {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

fn parse_input(input: &str) -> Engine {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let mut start = 0;
        let mut finish = 0;
        let mut in_number = false;

        for (column, c) in line.chars().enumerate() {
            match (in_number, c.is_ascii_digit()) {
                (false, true) => {
                    in_number = true;
                    start = column;
                    finish = column;
                }
                (true, true) => {
                    finish = column;
                }
                (true, false) => {
                    in_number = false;
                    numbers.push(Number {
                        row: row as i32,
                        column: start as i32,
                        length: (finish - start + 1) as i32,
                        value: line[start..=finish].parse::<u32>().unwrap(),
                    });
                }
                (false, false) => (),
            }

            if !in_number && c != '.' {
                symbols.push(Symbol {
                    row: row as i32,
                    column: column as i32,
                    maybe_gear: c == '*',
                })
            }
        }

        if in_number {
            numbers.push(Number {
                row: row as i32,
                column: start as i32,
                length: (finish - start + 1) as i32,
                value: line[start..=finish].parse::<u32>().unwrap(),
            });
        }
    }

    Engine { numbers, symbols }
}

fn is_any_adjacent(number: &Number, symbols: &[Symbol]) -> bool {
    symbols.iter().any(|symbol| is_adjacent(number, symbol))
}

fn is_adjacent(number: &Number, symbol: &Symbol) -> bool {
    symbol.row >= number.row - 1
        && symbol.row <= number.row + 1
        && symbol.column >= number.column - 1
        && symbol.column <= number.column + number.length
}

fn gear_ratio(numbers: &[Number], maybe_gear: &Symbol) -> Option<u32> {
    let adjacent_numbers = numbers
        .iter()
        .filter(|number| is_adjacent(number, maybe_gear))
        .collect::<Vec<&Number>>();

    if adjacent_numbers.len() == 2 {
        Some(
            adjacent_numbers
                .iter()
                .map(|&number| number.value)
                .product::<u32>(),
        )
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let engine = parse_input(input);

    Some(
        engine
            .numbers
            .into_iter()
            .filter(|number| is_any_adjacent(number, &engine.symbols))
            .map(|number| number.value)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let engine = parse_input(input);

    Some(
        engine
            .symbols
            .into_iter()
            .filter(|symbol| symbol.maybe_gear)
            .flat_map(|maybe_gear| gear_ratio(&engine.numbers, &maybe_gear))
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
