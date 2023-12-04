use itertools::Itertools;

advent_of_code::solution!(4);

struct Card {
    number: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (card, numbers_str) = value.split_once(": ").unwrap();
        let number = card.split_ascii_whitespace().collect_vec()[1].parse::<u32>().unwrap();
        let mut winning = vec![];
        let mut numbers = vec![];
        let (winning_str, n) = numbers_str.split_once(" | ").unwrap();
        winning_str.split_ascii_whitespace().for_each(|nb| {
            winning.push(nb.parse::<u32>().unwrap())
        });
        n.split_ascii_whitespace().for_each(|nb| {
            numbers.push(nb.parse::<u32>().unwrap())
        });
        Card { number, winning, numbers }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = input.lines()
        .map(|l| Card::from(l))
        .collect_vec();
    let score = cards.iter()
        .map(|c| {
            let w = c.numbers.iter().filter(|n| c.winning.contains(*n)).count();
            if w == 0 { 0 } else { 2_u32.pow((w - 1) as u32) }
        })
        .sum::<u32>();
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input.lines()
        .map(|l| Card::from(l))
        .collect_vec();
    let mut count: Vec<u32> = vec![1; cards.len()];
    cards.iter()
        .for_each(|c| {
            let w = c.numbers.iter().filter(|n| c.winning.contains(*n)).count();
            for i in (c.number)..(c.number + w as u32).min(cards.len() as u32)  {
                count[i as usize] += count[c.number as usize - 1]
            }
        });
    Some(count.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
