use std::cmp::{Ordering, Reverse};

use itertools::Itertools;

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

fn compare(a: &char, b: &char) -> Ordering {
    const ORDER: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    return ORDER
        .iter()
        .position(|character| character == a)
        .unwrap()
        .cmp(&ORDER.iter().position(|character| character == b).unwrap());
}

fn compare2(a: &char, b: &char) -> Ordering {
    const ORDER: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    return ORDER
        .iter()
        .position(|character| character == a)
        .unwrap()
        .cmp(&ORDER.iter().position(|character| character == b).unwrap());
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();

            (split.next().unwrap(), split.next().unwrap())
        })
        .sorted_by(|(a, _), (b, _)| {
            let mut groups_a: Vec<(char, usize)> = Vec::new();
            let mut groups_b: Vec<(char, usize)> = Vec::new();

            for (key, group) in &a
                .chars()
                .sorted_by(compare)
                .group_by(|character| *character)
            {
                groups_a.push((key, group.count()));
            }

            for (key, group) in &b
                .chars()
                .sorted_by(compare)
                .group_by(|character| *character)
            {
                groups_b.push((key, group.count()));
            }

            groups_a.sort_by_key(|el| Reverse(el.1));
            groups_b.sort_by_key(|el| Reverse(el.1));

            if (groups_a.iter().all(|el| el.1 == 1) && groups_b.iter().all(|el| el.1 == 1))
                || (groups_a.len() == groups_b.len()
                    && groups_a
                        .iter()
                        .map(|el| el.1)
                        .zip(groups_b.iter().map(|el| el.1))
                        .all(|(a, b)| a == b))
            {
                for (char_a, char_b) in a.chars().zip(b.chars()) {
                    match compare(&char_b, &char_a) {
                        Ordering::Equal => continue,
                        not_equal => return not_equal,
                    }
                }
            } else {
                for (count_a, count_b) in groups_a
                    .iter()
                    .map(|el| el.1)
                    .zip(groups_b.iter().map(|el| el.1))
                {
                    match count_a.cmp(&count_b) {
                        Ordering::Equal => continue,
                        not_equal => return not_equal,
                    }
                }
            }
            unreachable!()
        })
        .enumerate()
        .map(|(index, (_hand, wager))| (index + 1) * wager.parse::<usize>().unwrap())
        .sum::<usize>()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();

            (split.next().unwrap(), split.next().unwrap())
        })
        .sorted_by(|(a, _), (b, _)| {
            let mut groups_a: Vec<(char, usize)> = Vec::new();
            let mut groups_b: Vec<(char, usize)> = Vec::new();

            for (key, group) in &a
                .chars()
                .sorted_by(compare2)
                .group_by(|character| *character)
            {
                groups_a.push((key, group.count()));
            }

            for (key, group) in &b
                .chars()
                .sorted_by(compare2)
                .group_by(|character| *character)
            {
                groups_b.push((key, group.count()));
            }

            groups_a.sort_by_key(|el| Reverse(el.1));
            groups_b.sort_by_key(|el| Reverse(el.1));

            let (_, jokers) = groups_a
                .iter()
                .cloned()
                .find(|el| el.0 == 'J')
                .unwrap_or(('J', 0));

            if jokers != 0 && groups_a.len() != 1 {
                if groups_a[0].0 != 'J' {
                    groups_a[0].1 += jokers;
                } else {
                    groups_a[1].1 += jokers;
                }
                groups_a.retain(|el| el.0 != 'J')
            }

            let (_, jokers) = groups_b
                .iter()
                .cloned()
                .find(|el| el.0 == 'J')
                .unwrap_or(('J', 0));

            if jokers != 0 && groups_b.len() != 1 {
                if groups_b[0].0 != 'J' {
                    groups_b[0].1 += jokers;
                } else {
                    groups_b[1].1 += jokers;
                }
                groups_b.retain(|el| el.0 != 'J')
            }

            if (groups_a.iter().all(|el| el.1 == 1) && groups_b.iter().all(|el| el.1 == 1))
                || (groups_a.len() == groups_b.len()
                    && groups_a
                        .iter()
                        .map(|el| el.1)
                        .zip(groups_b.iter().map(|el| el.1))
                        .all(|(a, b)| a == b))
            {
                for (char_a, char_b) in a.chars().zip(b.chars()) {
                    match compare2(&char_b, &char_a) {
                        Ordering::Equal => continue,
                        not_equal => return not_equal,
                    }
                }
            } else {
                for (count_a, count_b) in groups_a
                    .iter()
                    .map(|el| el.1)
                    .zip(groups_b.iter().map(|el| el.1))
                {
                    match count_a.cmp(&count_b) {
                        Ordering::Equal => continue,
                        not_equal => return not_equal,
                    }
                }
            }

            unreachable!()
        })
        .enumerate()
        .map(|(index, (_hand, wager))| (index + 1) * wager.parse::<usize>().unwrap())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        let inp = include_str!("../input3.txt");
        assert_eq!(4466, part_1(inp));
        assert_eq!(4657, part_2(inp))
    }

    #[test]
    fn test_11() {
        let res = part_2(
            "2345J 1
J345A 1",
        );
        assert_eq!(res, 3);
    }
}
