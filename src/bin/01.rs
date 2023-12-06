advent_of_code::solution!(1);

fn compute_line(line: &str) -> u32 {
    let i1 = line.chars().find(|c| c.is_numeric())
        .unwrap_or('0').to_digit(10).unwrap();
    let i2 = line.chars().rev().find(|c| c.is_numeric())
        .unwrap_or('0').to_digit(10).unwrap();
    i1 * 10 + i2
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| compute_line(line))
        .sum()
    ;
    Some(sum)
}

fn transform_line(line: &str) -> String {
    let mut copy = line.to_string();
    let digits = [
        ("nine", "9"),
        ("eight", "8"),
        ("eightwo", "82"),
        ("eightwone", "821"),
        ("seven", "7"),
        ("six", "6"),
        ("five", "5"),
        ("four", "4"),
        ("three", "3"),
        ("two", "2"),
        ("twone", "21"),
        ("one", "1"),
        ("oneight", "18"),
    ];
    let mut replace = true;
    while replace {
        if let Some((t, i, o)) = digits
            .iter()
            .map(|(t, i)| (t, i, copy.find(t)))
            .filter(|(_, _, o)| o.is_some())
            .min_by_key(|(t, _, o)| o.unwrap() as isize - t.len() as isize)
        {
            let (p1, p2) = copy.split_at(o.unwrap() + t.len());
            copy = format!("{}{}", p1.replace(t, i), p2);
        } else {
            replace = false;
        }
    }
    copy
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|l| transform_line(l))
        .map(|l| compute_line(&l))
        .sum()
    ;
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ");
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
