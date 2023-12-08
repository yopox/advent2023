use std::collections::HashMap;
use std::ops::Deref;
use std::str::Lines;

use itertools::Itertools;

advent_of_code::solution!(8);

fn parse_input(input: Lines) -> HashMap<String, (String, String)> {
    let mut map = HashMap::new();
    input.for_each(|line| {
        let key = line[..3].to_string();
        let l = line[7..10].to_string();
        let r = line[12..15].to_string();
        map.insert(key, (l, r));
    });
    map
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let dir = lines.next().unwrap().chars().collect_vec();
    lines.next();
    let map = parse_input(lines);

    let mut current = &"AAA".to_string();
    let mut i = 0;
    let instr = dir.len();
    loop {
        if current == "ZZZ" { break; }
        let next = map.get(current).unwrap();
        current = match dir[i % instr] {
            'L' => &next.0,
            'R' => &next.1,
            _ => panic!("Wrong instr")
        };
        i += 1;
    }
    Some(i)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let dir = lines.next().unwrap().chars().collect_vec();
    lines.next();
    let map = parse_input(lines);
    let mut current = map.keys().filter(|k| k.ends_with('A')).collect_vec();
    let instr = dir.len();

    current.iter()
        .map(|c| {
            let mut c_i = c.clone();
            let mut i = 0;
            loop {
                if c_i.ends_with('Z') { break; }
                let next = map.get(c_i).unwrap();
                c_i = match dir[i % instr] {
                    'L' => &next.0,
                    _ => &next.1,
                };
                i += 1;
            }
            i
        })
        .reduce(|s1, s2| num_integer::lcm(s1, s2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");
        assert_eq!(result, Some(6));
    }
}
