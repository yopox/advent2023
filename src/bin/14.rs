use std::collections::HashSet;
use std::ops::Range;

use itertools::Itertools;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let (cubes, mut rocks, x_max, y_max) = parse_input(input);
    let y_ranges = compute_y_ranges(&cubes, x_max, y_max);
    move_up(&mut rocks, x_max, &y_ranges);
    rocks.iter().map(|(_, y)| y_max - *y + 1).sum1()
}

fn move_up(mut rocks: &mut HashSet<(usize, usize)>, x_max: usize, y_ranges: &Vec<Vec<Range<usize>>>) {
    for x in 0..=x_max {
        for range in &y_ranges[x] {
            let set = (range.start..range.end).map(|y| if rocks.contains(&(x, y)) { 1 } else { 0 }).sum::<usize>();
            (range.start..range.start + set).for_each(|y| { rocks.insert((x, y)); });
            (range.start + set..range.end).for_each(|y| { rocks.remove(&(x, y)); });
        }
    }
}

fn move_down(mut rocks: &mut HashSet<(usize, usize)>, x_max: usize, y_ranges: &Vec<Vec<Range<usize>>>) {
    for x in 0..=x_max {
        for range in &y_ranges[x] {
            let set = (range.start..range.end).map(|y| if rocks.contains(&(x, y)) { 1 } else { 0 }).sum::<usize>();
            (range.end - set..range.end).for_each(|y| { rocks.insert((x, y)); });
            (range.start..range.end - set).for_each(|y| { rocks.remove(&(x, y)); });
        }
    }
}

fn move_left(mut rocks: &mut HashSet<(usize, usize)>, y_max: usize, x_ranges: &Vec<Vec<Range<usize>>>) {
    for y in 0..=y_max {
        for range in &x_ranges[y] {
            let set = (range.start..range.end).map(|x| if rocks.contains(&(x, y)) { 1 } else { 0 }).sum::<usize>();
            (range.start..range.start + set).for_each(|x| { rocks.insert((x, y)); });
            (range.start + set..range.end).for_each(|x| { rocks.remove(&(x, y)); });
        }
    }
}

fn move_right(mut rocks: &mut HashSet<(usize, usize)>, y_max: usize, x_ranges: &Vec<Vec<Range<usize>>>) {
    for y in 0..=y_max {
        for range in &x_ranges[y] {
            let set = (range.start..range.end).map(|x| if rocks.contains(&(x, y)) { 1 } else { 0 }).sum::<usize>();
            (range.end - set..range.end).for_each(|x| { rocks.insert((x, y)); });
            (range.start..range.end - set).for_each(|x| { rocks.remove(&(x, y)); });
        }
    }
}

fn parse_input(input: &str) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>, usize, usize) {
    let mut cubes = HashSet::new();
    let mut rocks = HashSet::new();

    let mut x_max = 0;
    let mut y_max = 0;

    for (y, line) in input.lines().enumerate() {
        if y > y_max { y_max = y; }
        for (x, c) in line.chars().enumerate() {
            if x > x_max { x_max = x; }
            match c {
                '#' => { cubes.insert((x, y)); }
                'O' => { rocks.insert((x, y)); }
                _ => {}
            }
        }
    }
    (cubes, rocks, x_max, y_max)
}

fn compute_y_ranges(cubes: &HashSet<(usize, usize)>, x_max: usize, y_max: usize) -> Vec<Vec<Range<usize>>> {
    let mut y_ranges = vec![];
    for x in 0..=x_max {
        let mut r = vec![];
        let mut y = 0;
        for y_i in 0..=y_max {
            let blocked = cubes.contains(&(x, y_i));
            if blocked && y_i > y { r.push(y..y_i);y = y_i + 1; }
            else if blocked { y = y_i + 1; }
            else if y_i == y_max { r.push(y..y_i + 1); }
        }
        y_ranges.push(r);
    }
    y_ranges
}

fn compute_x_ranges(cubes: &HashSet<(usize, usize)>, x_max: usize, y_max: usize) -> Vec<Vec<Range<usize>>> {
    let mut x_ranges = vec![];
    for y in 0..=y_max {
        let mut r = vec![];
        let mut x = 0;
        for x_i in 0..=x_max {
            let blocked = cubes.contains(&(x_i, y));
            if blocked && x_i > x { r.push(x..x_i); x = x_i + 1; }
            else if blocked { x = x_i + 1; }
            else if x_i == x_max { r.push(x..x_i + 1); }
        }
        x_ranges.push(r);
    }
    x_ranges
}

pub fn part_two(input: &str) -> Option<usize> {
    let (cubes, mut rocks, x_max, y_max) = parse_input(input);
    let y_ranges = compute_y_ranges(&cubes, x_max, y_max);
    let x_ranges = compute_x_ranges(&cubes, x_max, y_max);

    let mut period = None;
    let mut i = 0;
    let mut scores = vec![];

    while i < 1000000000 {
        move_up(&mut rocks, x_max, &y_ranges);
        move_left(&mut rocks, x_max, &x_ranges);
        move_down(&mut rocks, x_max, &y_ranges);
        move_right(&mut rocks, x_max, &x_ranges);
        i += 1;

        let score = rocks.iter().map(|(_, y)| y_max - *y + 1).sum::<usize>();
        scores.push(score);
        if period == None && scores.len() > 10 {
            period = find_period(&scores);
            if let Some(p) = period {
                i += ((1000000000 - i) / p) * p;
            }
        }
    }

    Some(*scores.last().unwrap())
}

fn find_period(scores: &Vec<usize>) -> Option<usize> {
    let l = scores.len();
    for p_0 in 5..l/2 {
        if (0..p_0).all(|i| scores[l - 1 - i] == scores[l - 1 - p_0 - i]) { return Some(p_0) }
    }
    None
}

fn print_map(cubes: &HashSet<(usize, usize)>, rocks: &HashSet<(usize, usize)>, x_max: &usize, y_max: &usize) {
    for y in 0..=*y_max {
        for x in 0..=*x_max {
            if rocks.contains(&(x, y)) { print!("O"); } else if cubes.contains(&(x, y)) { print!("#"); } else { print!("."); }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
