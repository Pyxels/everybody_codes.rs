pub mod input;
pub use input::*;

pub fn run_part1() -> i64 {
    part1(INPUT_1)
}
pub fn run_part2() -> i64 {
    part2(INPUT_2)
}
pub fn run_part3() -> i64 {
    part3(INPUT_3)
}

pub fn part1(input: &str) -> i64 {
    input.chars().fold(0, |acc, letter| {
        acc + match letter {
            'A' => 0,
            'B' => 1,
            'C' => 3,
            x => unreachable!("'{x}'"),
        }
    })
}

pub fn part2(input: &str) -> i64 {
    input
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .fold(0, |acc, letters| {
            let bonus = if !letters.contains(&'x') { 2 } else { 0 };
            acc + bonus
                + match letters[0] {
                    'A' | 'x' => 0,
                    'B' => 1,
                    'C' => 3,
                    'D' => 5,
                    x => unreachable!("'{x}'"),
                }
                + match letters.get(1) {
                    None | Some('A') | Some('x') => 0,
                    Some('B') => 1,
                    Some('C') => 3,
                    Some('D') => 5,
                    x => unreachable!("'{x:?}'"),
                }
        })
}

pub fn part3(input: &str) -> i64 {
    input
        .chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .fold(0, |acc, letters| {
            let bonus = match letters.iter().filter(|l| *l == &'x').count() {
                3 => 0,
                2 => 0,
                1 => 2,
                0 => 6,
                x => unreachable!("'{x}'"),
            };
            acc + bonus
                + match letters[0] {
                    'A' | 'x' => 0,
                    'B' => 1,
                    'C' => 3,
                    'D' => 5,
                    x => unreachable!("'{x}'"),
                }
                + match letters[1] {
                    'A' | 'x' => 0,
                    'B' => 1,
                    'C' => 3,
                    'D' => 5,
                    x => unreachable!("'{x}'"),
                }
                + match letters.get(2) {
                    None | Some('A') | Some('x') => 0,
                    Some('B') => 1,
                    Some('C') => 3,
                    Some('D') => 5,
                    x => unreachable!("'{x:?}'"),
                }
        })
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "ABBAC";
    const INPUT2: &str = "AxBCDDCAxD";
    const INPUT3: &str = "xBxAAABCDxCC";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2), 28);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(INPUT3), 30);
    }
}

#[cfg(test)]
mod input_test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT_1), 1294);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_2), 5593);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(INPUT_3), 27434);
    }
}
