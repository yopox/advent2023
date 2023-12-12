use std::iter;

use itertools::Itertools;
use memoize::memoize;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    input.lines()
        .map(|l| {
            let (data, count) = l.split_once(" ").unwrap();
            (data, count.split(",").map(|p| p.parse::<usize>().unwrap()).collect_vec())
        })
        .map(|(data, groups)| {
            count(groups.clone(), data.chars().collect_vec())
        })
        .sum1()
}

#[memoize]
fn count(groups: Vec<usize>, input: Vec<char>) -> usize {
    let springs_sum = groups.iter().sum::<usize>();
    if springs_sum == 0 { return 0; }

    let remaining_areas = input.len();
    if springs_sum + groups.len() > remaining_areas + 1 { return 0; }

    let l = groups[0];
    let mut can_consume = input[0..l].iter().all(|c| *c != '.');
    let force_consume = input[0] == '#';

    // After consuming a group the next char must be '.' / end of input
    if can_consume {
        if groups.len() > 1 { can_consume = input[l] != '#'; }
        else {
            let all_consumed = remaining_areas <= l || input[l..].iter().all(|c| *c != '#');
            let valid = if all_consumed { 1 } else { 0 };
            return match force_consume {
                true => valid, // 1 group left, must consume
                false => valid + skip_and_count(groups, input), // can consume now + maybe later
            };
        }
    }

    return match can_consume {
        true => match force_consume {
            true => count(groups[1..].to_vec(), input[l+1..].to_vec()),
            false => count(groups[1..].to_vec(), input[l+1..].to_vec()) + skip_and_count(groups, input),
        }
        false => match force_consume {
            true => 0,
            false => skip_and_count(groups, input),
        }
    }
}

pub fn skip_and_count(groups: Vec<usize>, mut input: Vec<char>) -> usize {
    input.remove(0);
    count(groups, input)
}

pub fn part_two(input: &str) -> Option<usize> {
    input.lines()
        .map(|l| {
            let (data, count) = l.split_once(" ").unwrap();
            (data, count.split(",").map(|p| p.parse::<usize>().unwrap()).collect_vec())
        })
        .map(|(data, groups)| {
            count(groups.repeat(5), iter::repeat(data).take(5).join("?").chars().collect_vec())
        })
        .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
