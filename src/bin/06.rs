use std::str::Lines;

use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug)]
struct Race { time: usize, distance: usize }

fn from(value: &str) -> Vec<Race> {
    let mut lines = value.lines();
    let time = get_line_numbers(&mut lines);
    let distance = get_line_numbers(&mut lines);
    time.iter().zip(distance).map(|(t, d)| Race { time: *t, distance: d }).collect_vec()
}

fn get_line_numbers(lines: &mut Lines) -> Vec<usize> {
    lines.next().unwrap().split_ascii_whitespace().filter_map(|n| n.parse::<usize>().ok()).collect_vec()
}

fn get_line_number(lines: &mut Lines) -> usize {
    lines.next().unwrap().chars().filter(|c| c.is_numeric()).join("").parse::<usize>().unwrap()
}

impl Race {
    fn ways_to_beat(&self) -> usize {
        (0..self.time).map(|w| w * (self.time - w)).filter(|d| *d > self.distance).count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let races = from(input);
    races.iter().map(|r| r.ways_to_beat()).product1()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let time = get_line_number(&mut lines);
    let distance = get_line_number(&mut lines);
    let race = Race { time, distance };
    Some(race.ways_to_beat())
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
