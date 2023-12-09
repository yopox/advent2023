use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    input.lines()
        .map(|l| l.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect_vec())
        .map(|mut seq| {
            let mut res = 0;
            while !seq.iter().all(|i| *i == 0) {
                res += *seq.last().unwrap();
                seq = seq.windows(2).map(|l| match l {
                    &[i, j] => j - i,
                    _ => panic!(),
                }).collect_vec();
            }
            res
        })
        .sum1()
}

pub fn part_two(input: &str) -> Option<i32> {
    input.lines()
        .map(|l| l.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect_vec())
        .map(|mut seq| {
            let mut first = vec![];
            while !seq.iter().all(|i| *i == 0) {
                first.push(*seq.first().unwrap());
                seq = seq.windows(2).map(|l| match l {
                    &[i, j] => j - i,
                    _ => panic!(),
                }).collect_vec();
            }
            first.iter().rev().fold(0, |c, i| *i - c)
        })
        .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
