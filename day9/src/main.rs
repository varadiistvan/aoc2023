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

fn part_1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|el| el.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|first_values| vec![first_values])
        .map(|mut values| {
            let mut i = 0;

            while !values[i].iter().all(|el| el == &0) {
                let next = values[i]
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect();
                values.push(next);
                i += 1
            }

            values
                .iter()
                .rev()
                .fold(0, |acc, next| next.last().unwrap() + acc)
        })
        .sum()
}

fn part_2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|el| el.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|first_values| vec![first_values])
        .map(|mut values| {
            let mut i = 0;

            while !values[i].iter().all(|el| el == &0) {
                let next = values[i]
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect();
                values.push(next);
                i += 1
            }

            values.iter().rev().fold(0, |acc, next| next[0] - acc)
        })
        .sum()
}
