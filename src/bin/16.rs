use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(16);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn is_vertical(&self) -> bool { *self == Dir::Down || *self == Dir::Up }
    fn is_horizontal(&self) -> bool { *self == Dir::Left || *self == Dir::Right }

    fn delta(&self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }

    fn left(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    fn right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

fn check_pos(x: isize, y: isize, x_max: isize, y_max: isize) -> bool {
    !(x < 0 || x >= x_max || y < 0 || y >= y_max)
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let (x_max, y_max) = (map[0].len() as isize, map.len() as isize);
    let energized = simulate(((-1, 0), Dir::Right), &map, x_max, y_max);

    Some(energized)
}

fn simulate(initial: ((isize, isize), Dir), map: &Vec<Vec<char>>, x_max: isize, y_max: isize) -> usize {
    let mut energized: HashSet<(isize, isize)> = HashSet::new();
    let mut known: HashSet<((isize, isize), Dir)> = HashSet::new();
    let mut beams = vec![initial];
    while !beams.is_empty() {
        let ((x, y), dir) = beams.pop().unwrap();

        if check_pos(x, y, x_max, y_max) { energized.insert((x, y)); }

        let (dx, dy) = dir.delta();
        let (x1, y1) = (x + dx, y + dy);

        // OOB
        if !check_pos(x1, y1, x_max, y_max) { continue; }
        let (x1, y1) = (x1, y1);

        match map[y1 as usize][x1 as usize] {
            '.' => { beams.push(((x1, y1), dir)); }
            '/' => {
                if known.contains(&((x1, y1), dir)) { continue }
                known.insert(((x1, y1), dir));
                match dir {
                    Dir::Up | Dir::Down => beams.push(((x1, y1), dir.right())),
                    Dir::Left | Dir::Right => beams.push(((x1, y1), dir.left())),
                }
            }
            '\\' => {
                if known.contains(&((x1, y1), dir)) { continue }
                known.insert(((x1, y1), dir));
                match dir {
                    Dir::Up | Dir::Down => beams.push(((x1, y1), dir.left())),
                    Dir::Left | Dir::Right => beams.push(((x1, y1), dir.right())),
                }
            }
            '|' => {
                if known.contains(&((x1, y1), dir)) { continue }
                known.insert(((x1, y1), dir));
                if dir.is_vertical() { beams.push(((x1, y1), dir)); } else {
                    beams.push(((x1, y1), Dir::Up));
                    beams.push(((x1, y1), Dir::Down));
                }
            }
            '-' => {
                if known.contains(&((x1, y1), dir)) { continue }
                known.insert(((x1, y1), dir));
                if dir.is_horizontal() { beams.push(((x1, y1), dir)); } else {
                    beams.push(((x1, y1), Dir::Left));
                    beams.push(((x1, y1), Dir::Right));
                }
            }
            _ => {}
        }
    }
    energized.len()
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let (x_max, y_max) = (map[0].len() as isize, map.len() as isize);

    let mut results = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if y == 0 { results.insert(simulate(((x as isize, -1), Dir::Down), &map, x_max, y_max)); }
            if y == map.len() - 1 { results.insert(simulate(((x as isize, map.len() as isize), Dir::Up), &map, x_max, y_max)); }
            if x == 0 { results.insert(simulate(((-1, y as isize), Dir::Right), &map, x_max, y_max)); }
            if x == map[0].len() - 1 { results.insert(simulate(((map[0].len() as isize, y as isize), Dir::Left), &map, x_max, y_max)); }
        }
    }

    results.iter().max().map(|i| *i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
