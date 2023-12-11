use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

fn expand(n: isize, galaxies: &mut Vec<(isize, isize)>) {
    // Expand X
    let x_max = galaxies.iter().map(|(x, _)| x).max().unwrap();
    let x_e = (0..*x_max)
        .filter(|x_0| galaxies.iter().find(|(x, _)| *x == *x_0).is_none())
        .collect_vec();
    x_e
        .iter()
        .rev()
        .for_each(|x_0| galaxies.iter_mut().for_each(|(x, _)| { if *x > *x_0 { *x += n; } }));

    // Expand Y
    let y_max = galaxies.iter().map(|(_, y)| y).max().unwrap();
    let y_e = (0..*y_max)
        .filter(|y_0| galaxies.iter().find(|(_, y)| *y == *y_0).is_none())
        .collect_vec();
    y_e
        .iter()
        .rev()
        .for_each(|y_0| galaxies.iter_mut().for_each(|(_, y)| { if *y > *y_0 { *y += n; } }));
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut galaxies = parse_input(input);
    expand(1, &mut galaxies);
    compute_distances(&mut galaxies)
}

fn parse_input(input: &str) -> Vec<(isize, isize)> {
    input.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .flat_map(|(x, c)| match c {
                    '#' => Some((x as isize, y as isize)),
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec()
}

fn compute_distances(galaxies: &Vec<(isize, isize)>) -> Option<isize> {
    let mut distances = HashMap::new();
    for (i, g_1) in galaxies.iter().enumerate() {
        for (j, g_2) in galaxies.iter().enumerate() {
            if i == j || distances.contains_key(&(i, j)) || distances.contains_key(&(j, i)) { continue }
            let distance = (g_1.0 - g_2.0).abs() + (g_1.1 - g_2.1).abs();
            distances.insert((i, j), distance);
        }
    }

    distances.values().sum1()
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut galaxies = parse_input(input);
    expand(999999, &mut galaxies);
    compute_distances(&mut galaxies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
