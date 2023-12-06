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

fn main() {
    const FILE: &str = get_file();
    println!("Part 1: {}", part_1(FILE));
    println!("Part 2: {}", part_2(FILE));
}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let times = RE.find_iter(lines.next().unwrap());
    let records = RE.find_iter(lines.next().unwrap());

    times
        .zip(records)
        .map(|(timematch, recordmatch)| {
            (
                timematch.as_str().parse::<usize>().unwrap(),
                recordmatch.as_str().parse::<usize>().unwrap(),
            )
        })
        .map(|(time, record)| {
            (0..time)
                .filter(|held_time| (time - held_time) * held_time > record)
                .count()
        })
        .product()
}

fn part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let time = RE
        .find_iter(lines.next().unwrap())
        .map(|el| el.as_str())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let record = RE
        .find_iter(lines.next().unwrap())
        .map(|el| el.as_str())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    (0..time)
        .filter(|held_time| (time - held_time) * held_time > record)
        .count()
}
