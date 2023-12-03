use itertools::Itertools;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Number {
    n: u32,
    line: i32,
    start: i32,
    end: i32,
}

#[derive(Debug)]
struct Symbol {
    c: char,
    line: i32,
    pos: i32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers = vec![];
    let mut symbols = vec![];
    process_input(input, &mut numbers, &mut symbols);
    let sum = numbers
        .iter()
        .filter(|n| is_part(&symbols, *n))
        .map(|n| n.n)
        .sum::<u32>()
    ;
    Some(sum)
}

fn adjacent(n: &Number, s: &Symbol) -> bool {
    (n.line - 1..=n.line + 1).contains(&s.line) && (n.start - 1..=n.end + 1).contains(&s.pos)
}

fn is_part(mut symbols: &Vec<Symbol>, n: &Number) -> bool {
    symbols
        .iter()
        .find(|s| adjacent(n, *s))
        .is_some()
}

fn process_input(input: &str, mut numbers: &mut Vec<Number>, symbols: &mut Vec<Symbol>) {
    for (i, line) in input.lines().enumerate() {
        let mut nb = None;
        for (offset, char) in line.chars().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                match nb {
                    Some(n) => { nb = Some(n * 10 + digit); }
                    None => { nb = Some(digit); }
                }
            } else {
                process_number(&mut numbers, i, offset, &mut nb);
                nb = None;
                if char != '.' { symbols.push(Symbol { c: char, line: i as i32, pos: offset as i32 }); }
            }
        }
        process_number(&mut numbers, i, line.len(), &mut nb);
        nb = None;
    }
}

fn process_number(numbers: &mut Vec<Number>, line: usize, offset: usize, nb: &mut Option<u32>) {
    match nb {
        Some(n) => numbers.push(Number { n: *n, line: line as i32, start: offset as i32 - n.ilog10() as i32 - 1, end: offset as i32 - 1 }),
        None => {}
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut numbers = vec![];
    let mut symbols = vec![];
    process_input(input, &mut numbers, &mut symbols);
    let ratios = symbols.iter()
        .filter(|s| s.c == '*')
        .filter_map(|s| {
            let adjacent = numbers.iter()
                .filter(|n| adjacent(*n, s))
                .collect_vec();
            if adjacent.len() == 2 { Some(adjacent[0].n * adjacent[1].n) }
            else { None }
        })
        .sum()
    ;
    Some(ratios)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
