use std::cmp::min;

use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

pub fn part_one(input: &str) -> Option<usize> {
    input.split("\n\n")
        .map(|bloc| bloc.lines().map(|l| l.chars().collect_vec()).collect_vec())
        .map(|bloc| find_reflection(&bloc, 0))
        .map(|r| {
            match r {
                Reflection::Vertical(v) => v,
                Reflection::Horizontal(h) => h * 100,
            }
        })
        .sum1()
}

fn compare_vertical(input: &Vec<Vec<char>>, x1: usize, x2: usize) -> usize {
    (0..input.len()).map(|y| match input[y][x1] == input[y][x2] {
        true => 0,
        false => 1,
    }).sum()
}

fn compare_horizontal(input: &Vec<Vec<char>>, y1: usize, y2: usize) -> usize {
    (0..input[0].len()).map(|x| match input[y1][x] == input[y2][x] {
        true => 0,
        false => 1,
    }).sum()
}

fn find_reflection(input: &Vec<Vec<char>>, tolerance: usize) -> Reflection {
    // Horizontal
    for y in 0..input.len() - 1 {
        let mut diff = compare_horizontal(&input, y, y + 1);
        if diff <= tolerance {
            if diff == tolerance && y == 0 { return Reflection::Horizontal(1) }
            else {
                (0..min(y, input.len() - y - 2))
                    .for_each(|dy| { diff += compare_horizontal(&input, y - 1 - dy, y + 2 + dy); });
                if diff == tolerance { return Reflection::Horizontal(y + 1) }
            }
        }
    }

    // Vertical
    for x in 0..input[0].len() - 1 {
        let mut diff = compare_vertical(&input, x, x + 1);
        if diff <= tolerance {
            if diff == tolerance && x == 0 { return Reflection::Vertical(1) }
            else {
                (0..min(x, input[0].len() - x - 2))
                    .for_each(|dx| { diff += compare_vertical(&input, x - 1 - dx, x + 2 + dx); });
                if diff == tolerance { return Reflection::Vertical(x + 1) }
            }
        }
    }

    panic!("No reflection found")
}

pub fn part_two(input: &str) -> Option<usize> {
    input.split("\n\n")
        .map(|bloc| bloc.lines().map(|l| l.chars().collect_vec()).collect_vec())
        .map(|bloc| find_reflection(&bloc, 1))
        .map(|r| {
            match r {
                Reflection::Vertical(v) => v,
                Reflection::Horizontal(h) => h * 100,
            }
        })
        .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
