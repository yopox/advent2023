use itertools::Itertools;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    input
        .split(",")
        .map(|s| {
            let hash =  hash_str(s);
            hash
        })
        .sum1()
}

fn hash_str(input: &str) -> usize {
    let mut hash = 0;
    for c in input.chars() {
        if c == '\n' { continue }
        hash += c as usize;
        hash *= 17;
        hash = hash % 256;
    }
    hash
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    input
        .split(",")
        .for_each(|instr| {
            if instr.contains("-") {
                let lens = instr.strip_suffix("-").unwrap();
                let b = hash_str(lens);
                boxes[b].retain(|&(l, _)| l != lens);
            }
            else if instr.contains("=") {
                let (lens, fp) = instr.split_once("=").unwrap();
                let b = hash_str(lens);
                let fp = fp.replace("\n", "").parse::<usize>().unwrap();
                let mut replaced = false;
                boxes[b].iter_mut().for_each(|(box_lens, box_fp)| {
                   if lens == *box_lens { *box_fp = fp; replaced = true; }
                });
                if !replaced { boxes[b].push((lens, fp)); }
            }
        });

    boxes.iter().enumerate().map(|(i, lenses)|
        lenses.iter()
            .enumerate()
            .map(|(j, (_, fl))| {
                *fl * (j + 1)
            })
            .sum::<usize>()
        * (i + 1)
    ).sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash_str("HASH"), 52)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
