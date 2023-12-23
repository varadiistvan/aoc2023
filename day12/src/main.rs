use std::{hint::unreachable_unchecked, str::Chars};

use itertools::{GroupBy, Itertools};
use memoize::memoize;
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    str::ParallelString,
};

#[cfg(debug_assertions)]
const fn get_file() -> &'static str {
    include_str!("../input2.txt")
}

#[cfg(not(debug_assertions))]
const fn get_file() -> &'static str {
    include_str!("../input.txt")
}

fn main() {
    const FILE: &str = get_file();
    println!("Part 1: {}", part_1(FILE));
    println!("Part 2: {}", part_2(FILE));
}

fn part_1(input: &str) -> u128 {
    input
        .par_lines()
        .map(|line| {
            let mut chars = line.split_whitespace();

            let springs = chars.next().unwrap();
            let numbers = chars
                .next()
                .unwrap()
                .split(',')
                .map(|el| el.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let unknowns = springs.chars().filter(|el| el == &'?').count();

            let possibilities = 2usize.pow(unknowns as u32);

            (0..possibilities)
                .into_par_iter()
                .map(|i| {
                    let mut line_try = springs.to_owned();
                    for j in 0..unknowns {
                        line_try = line_try.replacen(
                            '?',
                            if (i / 2usize.pow(j as u32)) % 2 == 0 {
                                "."
                            } else {
                                "#"
                            },
                            1,
                        );
                    }
                    if is_valid(&line_try.chars().group_by(char::to_owned), &numbers) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u128>()
        })
        .sum()
}

fn is_valid<T: Fn(&char) -> char>(
    groups: &GroupBy<char, Chars<'_>, T>,
    numbers: &Vec<usize>,
) -> bool {
    let actual_groups: Vec<_> = groups
        .into_iter()
        .filter(|(key, _group)| key == &'#')
        .map(|(_key, group)| group.count())
        .collect();

    &actual_groups == numbers
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let springs = parts.next().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .split(',')
                .map(|el| el.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let numbers = numbers.repeat(5);

            let springs =
                springs.to_owned() + "?" + springs + "?" + springs + "?" + springs + "?" + springs;

            solve(springs.chars().collect_vec(), numbers)
        })
        .sum()
}

#[memoize]
fn solve(remaining_springs: Vec<char>, remaining_numbers: Vec<usize>) -> usize {
    if remaining_numbers.is_empty() {
        if remaining_springs.is_empty() || remaining_springs.iter().all(|c| *c == '.' || *c == '?')
        {
            1
        } else {
            0
        }
    } else if remaining_springs.is_empty() {
        0
    } else {
        match remaining_springs[0] {
            '.' => next_good(remaining_springs, remaining_numbers),
            '#' => next_broken(remaining_springs, remaining_numbers),
            '?' => next_unknown(remaining_springs, remaining_numbers),
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

fn next_good(remaining_springs: Vec<char>, remaining_numbers: Vec<usize>) -> usize {
    solve(
        remaining_springs
            .iter()
            .skip_while(|c| **c == '.')
            .copied()
            .collect_vec(),
        remaining_numbers,
    )
}

fn next_broken(remaining_springs: Vec<char>, remaining_numbers: Vec<usize>) -> usize {
    let number = remaining_numbers[0];
    if remaining_springs.len() >= number
        && remaining_springs
            .iter()
            .take(number)
            .all(|c| *c == '#' || *c == '?')
    {
        let remaining_springs = remaining_springs[number..].to_vec();
        let remaining_numbers = remaining_numbers[1..].to_vec();
        if !remaining_springs.is_empty() {
            if remaining_springs[0] == '.' || remaining_springs[0] == '?' {
                solve(remaining_springs[1..].to_vec(), remaining_numbers)
            } else {
                0
            }
        } else {
            solve(remaining_springs, remaining_numbers)
        }
    } else {
        0
    }
}

fn next_unknown(remaining_springs: Vec<char>, remaining_numbers: Vec<usize>) -> usize {
    let mut acc = solve(remaining_springs[1..].to_vec(), remaining_numbers.clone());
    let number = remaining_numbers[0];
    if remaining_springs.len() >= number
        && remaining_springs
            .iter()
            .take(number)
            .all(|c| *c == '#' || *c == '?')
    {
        let remaining_springs = remaining_springs[number..].to_vec();
        let remaining_numbers = remaining_numbers[1..].to_vec();
        if !remaining_springs.is_empty() {
            if remaining_springs[0] == '.' || remaining_springs[0] == '?' {
                acc += solve(remaining_springs[1..].to_vec(), remaining_numbers);
            }
        } else {
            acc += solve(remaining_springs, remaining_numbers);
        }
    }
    acc
}
