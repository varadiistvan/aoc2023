use std::fmt::Display;

use once_cell::sync::Lazy;
use regex::{Match, Regex};

#[cfg(debug_assertions)]
const fn get_file() -> &'static str {
    include_str!("../input2.txt")
}

#[cfg(not(debug_assertions))]
const fn get_file() -> &'static str {
    include_str!("../input.txt")
}

static SYMBOL: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\d\.]").unwrap());
static NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
static STAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\*").unwrap());

fn main() {
    const FILE: &str = get_file();
    println!("Part 1: {}", part_1(FILE));
    println!("Part 2: {}", part_2(FILE));
}

fn part_1(input: &str) -> u128 {
    let line_size = input
        .lines()
        .max_by(|el1, el2| el1.len().cmp(&el2.len()))
        .unwrap()
        .len();
    let lines: Vec<_> = vec!["."; line_size]
        .join("")
        .lines()
        .chain(input.lines())
        .chain(vec!["."; line_size].join("").lines())
        .map(|el| [".", el, &vec!["."; line_size + 1 - el.len()].join("")].join(""))
        .collect();

    let a = lines.windows(3).map(|window| {
        let (top, middle, bottom) = (&window[0], &window[1], &window[2]);

        NUMBER
            .find_iter(middle)
            .filter_map(|number| {
                if top[number.range().start - 1..number.range().end + 1]
                    .chars()
                    .chain(bottom[number.range().start - 1..number.range().end + 1].chars())
                    .chain(middle[number.range().start - 1..number.range().start].chars())
                    .chain(middle[number.range().end..number.range().end + 1].chars())
                    .any(|el| SYMBOL.is_match(&el.to_string()))
                {
                    Some(number.as_str().parse::<u128>().unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    });

    a.flatten().collect::<Vec<_>>().into_iter().sum::<u128>()
}

fn part_2(input: &str) -> impl Display {
    let line_size = input
        .lines()
        .max_by(|el1, el2| el1.len().cmp(&el2.len()))
        .unwrap()
        .len();
    let lines: Vec<_> = vec!["."; line_size]
        .join("")
        .lines()
        .chain(input.lines())
        .chain(vec!["."; line_size].join("").lines())
        .map(|el| [".", el, &vec!["."; line_size + 1 - el.len()].join("")].join(""))
        .collect();

    lines
        .windows(3)
        .map(|window| {
            let (top, middle, bottom) = (&window[0], &window[1], &window[2]);

            STAR.find_iter(middle)
                .filter_map(|star| {
                    let index = star.range().start;

                    let callback = |number: &Match| {
                        (number.range().start - 1..number.range().end + 1).contains(&index)
                    };

                    let nums = NUMBER
                        .find_iter(top)
                        .filter(callback)
                        .chain(NUMBER.find_iter(middle).filter(callback))
                        .chain(NUMBER.find_iter(bottom).filter(callback))
                        .map(|el| el.as_str().parse().unwrap())
                        .collect::<Vec<_>>();

                    if nums.len() == 2 {
                        Some(nums.iter().product::<u128>())
                    } else {
                        None
                    }
                })
                .sum::<u128>()
        })
        .sum::<u128>()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_1() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        assert_eq!(part_1(input), 4361);
    }

    #[test]
    fn test_case_2() {
        let input = "..1..\n.2*3.\n..4..";
        assert_eq!(part_1(input), 10);
    }

    #[test]
    fn test_case_3() {
        let input = "1111\n*1*1\n1111";
        assert_eq!(part_1(input), 2224);
    }

    #[test]
    fn test_case_4() {
        let input = "..5..\n.*6*.\n..5..";
        assert_eq!(part_1(input), 16);
    }

    #[test]
    fn test_case_5() {
        let input = ".12.\n.*..\n.34.";
        assert_eq!(part_1(input), 46);
    }

    #[test]
    fn test_case_6() {
        let input = "..........\n.........1\n..........";
        assert_eq!(part_1(input), 0);
    }

    #[test]
    fn test_case_7() {
        let input = "-1";
        assert_eq!(part_1(input), 1);
    }

    #[test]
    fn test_case_8() {
        let input = "$..\n.11\n.11\n$..\n..$\n11.\n11.\n..$";
        assert_eq!(part_1(input), 44);
    }

    #[test]
    fn test_case_9() {
        let input = "11.$.";
        assert_eq!(part_1(input), 0);
    }

    #[test]
    fn test_case_10() {
        let input = "$11\n...\n11$\n...";

        assert_eq!(part_1(input), 22);
    }

    #[test]
    fn test_case_11() {
        let input = "$......$\n.11..11.\n.11.11..\n$......$";

        assert_eq!(part_1(input), 33);
    }

    #[test]
    fn test_case_12() {
        let input = ".......5......\n..7*..*.....4*\n...*13*......9\n.......15.....\n..............\n..............\n..............\n..............\n..............\n..............\n21............\n...*9.........";

        assert_eq!(part_1(input), 62);
    }

    #[test]
    fn test_case_13() {
        let input = "........\n.24$-4..\n......*.";

        assert_eq!(part_1(input), 28);
    }

    #[test]
    fn test_case_14() {
        let input = "97..\n...*\n100.";

        assert_eq!(part_1(input), 100);
    }
}
