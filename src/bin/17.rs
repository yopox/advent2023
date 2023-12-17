use std::ops::Range;
use std::sync::RwLock;

use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

advent_of_code::solution!(17);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn delta(&self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }
}

#[derive(Clone)]
struct Dist {
    up: usize, right: usize, down: usize, left: usize,
}

impl Default for Dist {
    fn default() -> Self {
        Self {
            up: usize::MAX,
            right: usize::MAX,
            down: usize::MAX,
            left: usize::MAX,
        }
    }
}

impl Dist {
    fn get(&self, dir: &Dir) -> usize {
        match dir {
            Dir::Up => self.up,
            Dir::Down => self.down,
            Dir::Left => self.left,
            Dir::Right => self.right,
        }
    }

    fn set(&mut self, dir: &Dir, val: usize) {
        match dir {
            Dir::Up => self.up = val,
            Dir::Down => self.down = val,
            Dir::Left => self.left = val,
            Dir::Right => self.right = val,
        }
    }

    fn min(&self) -> usize {
        *[self.right, self.up, self.left, self.down].iter().min().unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|l| l.chars()
            .map(|n| n as usize - '0' as usize)
            .collect_vec()
        )
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let numbers = parse_input(input);
    calculate_min_distance(numbers, 1..4)
}

fn calculate_min_distance(numbers: Vec<Vec<usize>>, range: Range<usize>) -> Option<usize> {
    let mut distances: Vec<Vec<RwLock<Dist>>> = (0..numbers.len()).map(|_| (0..numbers[0].len()).map(|_| RwLock::new(Dist::default())).collect_vec()).collect_vec();
    if let Ok(mut d) = distances[0][0].write() { d.up = 0; d.down = 0; d.left = 0; d.right = 0; };
    let mut changed = RwLock::new(true);

    while *changed.read().unwrap() {
        let new_dist = distances.iter().map(|l| l.iter().map(|lock| RwLock::new(lock.read().unwrap().clone())).collect_vec()).collect_vec();
        changed = RwLock::new(false);
        (0..numbers.len()).into_par_iter().for_each(|y| {
            (0..numbers[0].len()).into_par_iter().for_each(|x| {
                let dist = &distances[y][x];
                for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                    let total = dist.read().unwrap().get(&dir);
                    if total == usize::MAX { continue }

                    let mut dirs = match dir {
                        Dir::Up | Dir::Down => [Dir::Left, Dir::Right],
                        Dir::Left | Dir::Right => [Dir::Up, Dir::Down],
                    };
                    if total == 0 { dirs = [Dir::Down, Dir::Right]; }

                    for amount in range.start..range.end {
                        for d1 in dirs {
                            let valid = match d1 {
                                Dir::Up => y >= amount,
                                Dir::Down => y < numbers.len() - amount,
                                Dir::Left => x >= amount,
                                Dir::Right => x < numbers[0].len() - amount,
                            };
                            if !valid { continue }
                            let (dx, dy) = d1.delta();
                            let x1 = (x as isize + dx * amount as isize) as usize;
                            let y1 = (y as isize + dy * amount as isize) as usize;
                            let d = (1..=amount).map(|j|
                                numbers[(y as isize + dy * j as isize) as usize][(x as isize + dx * j as isize) as usize]
                            ).sum::<usize>() + total;
                            if distances[y1][x1].read().unwrap().get(&d1) > d {
                                new_dist[y1][x1].write().unwrap().set(&d1, d);
                                if !*changed.read().unwrap() { let mut c = changed.write().unwrap(); *c = true; }
                            }
                        }
                    }
                }
            });
        });
        distances = new_dist;
    }

    let shortest_dist = Some(distances.last().unwrap().last().unwrap().read().unwrap().min());
    shortest_dist
}

pub fn part_two(input: &str) -> Option<usize> {
    let numbers = parse_input(input);
    calculate_min_distance(numbers, 4..11)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
