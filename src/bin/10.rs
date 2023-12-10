use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Dir { N, S, E, W }

impl Dir {
    fn apply(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Dir::N => (x, y - 1),
            Dir::S => (x, y + 1),
            Dir::E => (x + 1, y),
            Dir::W => (x - 1, y),
        }
    }

    fn is_possible(&self, pos: (usize, usize), map: &Vec<Vec<char>>) -> bool {
        match self {
            Dir::N => pos.1 > 0 && ['|', '7', 'F'].contains(&map[pos.1 - 1][pos.0]),
            Dir::S => pos.1 + 1 < map.len() && ['|', 'L', 'J'].contains(&map[pos.1 + 1][pos.0]),
            Dir::E => pos.0 + 1 < map[0].len() && ['-', 'J'].contains(&map[pos.1][pos.0 + 1]),
            Dir::W => pos.0 > 0 && ['-', 'L', 'F'].contains(&map[pos.1][pos.0 - 1]),
        }
    }
}

fn step(pos: (usize, usize), map: &Vec<Vec<char>>, dir: &Dir) -> Option<Dir> {
    match (map[pos.1][pos.0], dir) {
        ('|', Dir::N) => Some(Dir::N),
        ('|', Dir::S) => Some(Dir::S),
        ('-', Dir::E) => Some(Dir::E),
        ('-', Dir::W) => Some(Dir::W),
        ('L', Dir::S) => Some(Dir::E),
        ('L', Dir::W) => Some(Dir::N),
        ('J', Dir::S) => Some(Dir::W),
        ('J', Dir::E) => Some(Dir::N),
        ('7', Dir::N) => Some(Dir::W),
        ('7', Dir::E) => Some(Dir::S),
        ('F', Dir::N) => Some(Dir::E),
        ('F', Dir::W) => Some(Dir::S),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input.lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec()
    ;
    let (x, y) = find_start(&map);
    let start_dir = [Dir::S, Dir::E, Dir::N, Dir::W].iter()
        .filter(|d| d.is_possible((x, y), &map))
        .collect_vec()
    ;

    let mut distances = HashMap::new();
    distances.insert((x, y), 0);
    for dir in start_dir.iter() {
        let mut next_dir = Some(**dir);
        let mut pos = (x, y);
        let mut i = 0;
        while next_dir.is_some() {
            i += 1;
            pos = next_dir.as_ref().unwrap().apply(pos.0, pos.1);
            if let Some(i_0) = distances.get(&pos) {
                distances.insert(pos, i.min(*i_0));
            } else {
                distances.insert(pos, i);
            }
            next_dir = step(pos, &map, &next_dir.unwrap());
        }
    }

    Some(*distances.values().max().unwrap())
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    panic!("no start")
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input.lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec()
    ;

    // Determine start dir
    let (x, y) = find_start(&map);
    let start_dir = [Dir::S, Dir::E, Dir::N, Dir::W].iter()
        .find(|d| d.is_possible((x, y), &map))
        .unwrap()
    ;

    // Compute loop
    let mut loop_tiles = HashSet::new();
    let mut loop_dir = HashMap::new();
    let mut dir = Some(*start_dir);
    let mut turns = 0;
    let mut pos = (x, y);
    while let Some(d) = dir {
        pos = d.apply(pos.0, pos.1);
        loop_tiles.insert(pos);
        loop_dir.insert(pos, d);
        turns += match (map[pos.1][pos.0], d) {
            ('L', Dir::W) => 1,
            ('L', Dir::S) => -1,
            ('J', Dir::S) => 1,
            ('J', Dir::E) => -1,
            ('7', Dir::E) => 1,
            ('7', Dir::N) => -1,
            ('F', Dir::N) => 1,
            ('F', Dir::W) => -1,
            _ => 0,
        };
        dir = step(pos, &map, &dir.unwrap());
    }

    // We enclose left tiles if there are more left turns
    let left = turns < 0;

    // Fill with water
    let mut contained: HashSet<(usize, usize)> = HashSet::new();
    loop_tiles.iter()
        .flat_map(|&(x, y)| match (map[y][x], loop_dir.get(&(x, y)).unwrap(), left) {
            ('-', Dir::E, true) | ('-', Dir::W, false) => vec![Dir::N.apply(x, y)],
            ('-', Dir::W, true) | ('-', Dir::E, false)  => vec![Dir::S.apply(x, y)],
            ('|', Dir::N, true) | ('|', Dir::S, false) => vec![Dir::W.apply(x, y)],
            ('|', Dir::S, true) | ('|', Dir::N, false) => vec![Dir::E.apply(x, y)],
            ('L', Dir::W, true) => vec![Dir::W.apply(x, y), Dir::S.apply(x, y)],
            ('L', Dir::S, false) => vec![Dir::W.apply(x, y), Dir::S.apply(x, y)],
            ('J', Dir::S, true) => vec![Dir::E.apply(x, y), Dir::S.apply(x, y)],
            ('J', Dir::E, false) => vec![Dir::E.apply(x, y), Dir::S.apply(x, y)],
            ('7', Dir::E, true) => vec![Dir::N.apply(x, y), Dir::E.apply(x, y)],
            ('7', Dir::N, false) => vec![Dir::N.apply(x, y), Dir::E.apply(x, y)],
            ('F', Dir::N, true) => vec![Dir::W.apply(x, y), Dir::N.apply(x, y)],
            ('F', Dir::W, false) => vec![Dir::W.apply(x, y), Dir::N.apply(x, y)],
            _ => vec![],
        })
        .for_each(|(x, y)| {
            if !loop_tiles.contains(&(x, y)) { contained.insert((x, y)); }
        })
    ;

    let mut changed = true;
    while changed {
        changed = false;
        let mut new = HashSet::new();
        for tile in &contained {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (x, y) = (tile.0 as isize + dx, tile.1 as isize + dy);
                if x < 0 || y < 0 { continue }
                let (x, y) = (x as usize, y as usize);
                if x < map[0].len() && y < map.len() {
                    if !contained.contains(&(x, y)) && !loop_tiles.contains(&(x, y)) {
                        changed = true;
                        new.insert((x, y));
                    }
                }
            }
        }
        new.iter().for_each(|t| { contained.insert(*t); });
    }

    Some(contained.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L");
        assert_eq!(result, Some(10));
    }
}
