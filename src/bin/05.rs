advent_of_code::solution!(5);

use core::ops::RangeInclusive;

struct MappingRange {
    source: RangeInclusive<u64>,
    destination_start: u64,
}

impl MappingRange {
    fn get_destination(&self, source: u64) -> Option<u64> {
        if self.source.contains(&source) {
            Some(self.destination_start + (source - self.source.start()))
        } else {
            None
        }
    }
}

struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn get_destination(&self, source: u64) -> u64 {
        self.ranges
            .iter()
            .flat_map(|range| range.get_destination(source))
            .take(1)
            .next()
            .unwrap_or(source)
    }
}

struct Almanac {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn get_location(&self, seed: u64) -> u64 {
        self.mappings
            .iter()
            .fold(seed, |acc, range| range.get_destination(acc))
    }
}

fn parse_input(input: &str) -> Almanac {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .map(|s| s.split(' '))
        .unwrap()
        .map(|s| s.trim())
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    lines.next();

    let mappings = lines
        .collect::<Vec<_>>()
        .split(|&line| line.is_empty())
        .map(|m| {
            m.iter()
                .skip(1)
                .map(|&mapping| {
                    mapping
                        .split_whitespace()
                        .map(|i| i.parse::<u64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|range| MappingRange {
                    source: range[1]..=range[1] + range[2],
                    destination_start: range[0],
                })
                .collect::<Vec<_>>()
        })
        .map(|ranges| Mapping { ranges })
        .collect::<Vec<_>>();

    Almanac { seeds, mappings }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = parse_input(input);

    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.get_location(seed))
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = parse_input(input);

    almanac
        .seeds
        .chunks_exact(2)
        .map(|c| c[0]..c[0] + c[1])
        .flat_map(|seeds| seeds.map(|seed| almanac.get_location(seed)))
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
