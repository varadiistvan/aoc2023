use std::collections::VecDeque;

use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;

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

static EMPTY_LINE: Lazy<Regex> = Lazy::new(|| Regex::new(r".+:").unwrap());

fn part_1(input: &str) -> usize {
    let mut parts = EMPTY_LINE
        .split(input)
        .map(|el| {
            el.lines()
                .filter(|el| !el.is_empty())
                .map(|el| {
                    el.split_whitespace()
                        .filter_map(|el| el.parse().ok())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>()
        })
        .filter(|el| !el.is_empty())
        .collect::<VecDeque<Vec<Vec<usize>>>>();
    assert_eq!(parts.len(), 8);

    let seeds = parts
        .pop_front()
        .unwrap()
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let ranges = parts
        .into_iter()
        .map(|part| {
            part.into_iter()
                .map(|range_dec| {
                    (
                        range_dec[0]..range_dec[0] + range_dec[2],
                        range_dec[1]..range_dec[1] + range_dec[2],
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    seeds
        .into_iter()
        .map(|seed| {
            ranges.iter().fold(seed, |acc, ranges| {
                let containing = ranges
                    .iter()
                    .cloned()
                    .find(|el| el.1.contains(&acc))
                    .unwrap_or((acc..acc + 1, acc..acc + 1));
                containing.0.start + (acc - containing.1.start)
            })
        })
        .min()
        .unwrap()
}

fn part_2(input: &str) -> usize {
    let parts = EMPTY_LINE
        .split(input)
        .map(|el| {
            el.lines()
                .filter(|el| !el.is_empty())
                .map(|el| {
                    el.split_whitespace()
                        .filter_map(|el| el.parse().ok())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>()
        })
        .filter(|el| !el.is_empty())
        .collect::<Vec<Vec<Vec<usize>>>>();
    assert_eq!(parts.len(), 8);

    let ranges = parts[1..]
        .iter()
        .map(|part| {
            part.iter()
                .map(|range_dec| {
                    (
                        range_dec[0]..range_dec[0] + range_dec[2],
                        range_dec[1]..range_dec[1] + range_dec[2],
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    parts[0]
        .par_iter() // Use parallel iterator
        .flatten()
        .collect::<Vec<_>>()
        .par_chunks(2)
        .map(|el| *el[0]..(el[0] + el[1]))
        .map(|seeds| {
            seeds
                .map(|seed| {
                    ranges.iter().fold(seed, |acc, ranges| {
                        let containing = ranges
                            .iter()
                            .cloned()
                            .find(|el| el.1.contains(&acc))
                            .unwrap_or((acc..acc + 1, acc..acc + 1));
                        containing.0.start + (acc - containing.1.start)
                    })
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}
