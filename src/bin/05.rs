use std::ops::Range;

use itertools::Itertools;

advent_of_code::solution!(5);

struct Converter {
    rules: Vec<Rule>,
}

impl Converter {
    fn convert_min(&self, n: usize) -> usize {
        match self.rules.iter()
            .map(|rule| rule.convert(n))
            .find(|r| *r != n) {
            Some(r) => r,
            None => n,
        }
    }
    fn convert_range(&self, range: &Range<usize>) -> Vec<Range<usize>> {
        let mut old = vec![range.clone()];
        let mut new = vec![];
        for rule in &self.rules {
            let mut old_2 = vec![];
            for r in old {
                match rule.convert_range(&r) {
                    Some((converted_range, new_range)) => {
                        new.push(new_range);
                        if converted_range.start > r.start { old_2.push(range.start..converted_range.start); }
                        if converted_range.end < r.end { old_2.push(converted_range.end..range.end); }
                    }
                    None => { old_2.push(r); }
                }
            }
            old = old_2;
        }
        // println!("range: {:?} -> old {:?} /  new {:?}", range, old, new);
        new.append(&mut old);
        new
    }
}

struct Rule {
    start_1: usize,
    start_2: usize,
    len: usize,
}

impl Rule {
    fn convert(&self, n: usize) -> usize {
        if n < self.start_1 { return n }
        if n >= self.start_1 + self.len { return n }
        return self.start_2 + n - self.start_1
    }

    fn convert_range(&self, r: &Range<usize>) -> Option<(Range<usize>, Range<usize>)> {
        let start = r.start.max(self.start_1);
        if start < r.end && start < self.start_1 + self.len {
            let end = r.end.min(self.start_1 + self.len);
            let len = end - start;
            let mapped_start = self.start_2 + start - self.start_1;
            Some((start..end, mapped_start..mapped_start+len))
        } else {
            None
        }
    }
}

impl From<&str> for Converter {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        if let Some(_) = lines.next() {
            let rules = lines.map(|l| Rule::from(l)).collect_vec();
            Self { rules }
        } else {
            panic!("Malformed map");
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let nb = value
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();
        if nb.len() != 3 { panic!("Malformed rule"); }
        Self {
            start_1: nb[1],
            start_2: nb[0],
            len: nb[2],
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (seeds, converters) = parse_input(input);

    seeds.iter()
        .map(|seed| convert_seed(&converters, seed))
        .min()
}

fn convert_seed(converters: &Vec<Converter>, seed: &usize) -> usize {
    converters
        .iter()
        .fold(*seed, |acc, c| c.convert_min(acc))
}

fn convert_seed_range(converters: &Vec<Converter>, range: &Range<usize>) -> Vec<Range<usize>> {
    converters
        .iter()
        .fold(vec![range.clone()], |acc, c| acc.iter().flat_map(|r| c.convert_range(r)).collect_vec())
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Converter>) {
    let mut parts = input.split("\n\n");
    let seeds = parts
        .next().unwrap()
        .split_once(": ").unwrap().1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    let converters = parts
        .map(|part| Converter::from(part))
        .collect_vec();
    (seeds, converters)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (seeds, converters) = parse_input(input);
    seeds
        .chunks(2).into_iter()
        .flat_map(|arr| match arr {
            [s1, s2] => convert_seed_range(&converters, &(*s1..*s1+*s2)),
            _ => panic!(),
        })
        .map(|r| r.start)
        .min()
}

#[test]
fn test() {
    let r = convert_seed_range(&vec![Converter { rules: vec![
        Rule { start_1: 56, start_2: 60, len: 37, },
        Rule { start_1: 93, start_2: 56, len: 4, },
    ],  }], &(90..99));
    println!("{:?}", r);
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
