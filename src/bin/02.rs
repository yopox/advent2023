advent_of_code::solution!(2);

struct Game {
    id: usize,
    played: Vec<(usize, usize, usize)>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (id_str, content) = value.split_once(": ").expect("No :");
        let id = id_str.split_once(" ").expect("bad ID")
            .1
            .parse::<usize>().expect("ID is not a number");

        let mut played = vec![];
        for set in content.split("; ") {
            let mut current = (0, 0, 0);
            for cubes in set.split(", ") {
                let (q_str, color) = cubes.split_once(" ").expect("bad cubes");
                let q = q_str.parse::<usize>().expect("Amount of cubes is not a number");
                match color {
                    "red" => { current.0 += q; }
                    "green" => { current.1 += q; }
                    "blue" => { current.2 += q; }
                    _ => panic!("unknown color")
                }
            }
            played.push(current);
        }

        Game { id, played }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let r_max = 12;
    let g_max = 13;
    let b_max = 14;
    let games = input
        .lines()
        .map(|line| Game::from(line))
        .filter(|game|
            game.played.iter().max_by_key(|(r, _, _)| *r).unwrap_or(&(0, 0, 0)).0 <= r_max
                && game.played.iter().max_by_key(|(_, g, _)| *g).unwrap_or(&(0, 0, 0)).1 <= g_max
                && game.played.iter().max_by_key(|(_, _, b)| *b).unwrap_or(&(0, 0, 0)).2 <= b_max
        )
        .map(|game| game.id)
        .sum::<usize>()
    ;

    Some(games)
}

pub fn part_two(input: &str) -> Option<usize> {
    let games = input
        .lines()
        .map(|line| Game::from(line))
        .map(|game|
            (game.played.iter().max_by_key(|(r, _, _)| *r).unwrap_or(&(0, 0, 0)).0,
             game.played.iter().max_by_key(|(_, g, _)| *g).unwrap_or(&(0, 0, 0)).1,
             game.played.iter().max_by_key(|(_, _, b)| *b).unwrap_or(&(0, 0, 0)).2, )
        )
        .map(|(r, g, b)| r * g * b)
        .sum::<usize>()
    ;

    Some(games)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
