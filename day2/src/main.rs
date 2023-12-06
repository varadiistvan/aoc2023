use std::{
    fmt::{Display, Formatter},
    iter::Sum,
    ops::Add,
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
    // println!("Part 1: {}", part_1(file));
    println!("Part 2: {}", part_2(FILE));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Blue => write!(f, "blue"),
        }
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Invalid color"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Result(u32, u32, u32);

impl Add for Result {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sum for Result {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self(0, 0, 0), |acc, x| acc + x)
    }
}

fn part_1(file: &str) -> u32 {
    const AMOUNTS: [(Color, u32); 3] = [(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)];

    file.lines()
        .filter_map(|line| {
            let mut split = line.split(':');

            let game = split.next()?[5..].parse::<u32>().ok()?;

            if split
                .next()?
                .split(';')
                .map(|draw| {
                    draw.split(',').filter_map(|draw_parts| {
                        let mut parts = draw_parts.split_whitespace();
                        Some((
                            parts.next()?.parse::<u32>().ok()?,
                            Color::from(parts.next()?),
                        ))
                    })
                })
                .all(|el| {
                    let amounts: Result = el
                        .map(|(num, color)| match color {
                            Color::Red => Result(num, 0, 0),
                            Color::Green => Result(0, num, 0),
                            Color::Blue => Result(0, 0, num),
                        })
                        .sum();
                    AMOUNTS.iter().all(|(color, limit)| match color {
                        Color::Red => amounts.0 <= *limit,
                        Color::Green => amounts.1 <= *limit,
                        Color::Blue => amounts.2 <= *limit,
                    })
                })
            {
                Some(game)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(file: &str) -> u128 {
    file.lines()
        .map(|game| {
            let relevant = game.split(':').last().unwrap();

            let sure = relevant
                .split(';')
                .map(|draw| {
                    let interpreted = draw.split(',').map(|individual| {
                        let mut parts = individual.split_whitespace();

                        (
                            parts.next().unwrap().parse::<u128>().unwrap(),
                            Color::from(parts.next().unwrap()),
                        )
                    });

                    interpreted.fold(
                        ((Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)),
                        |acc, (num, color)| match color {
                            Color::Red => ((Color::Red, acc.0 .1 + num), acc.1, acc.2),
                            Color::Green => (acc.0, (Color::Green, acc.1 .1 + num), acc.2),
                            Color::Blue => (acc.0, acc.1, (Color::Blue, acc.2 .1 + num)),
                        },
                    )
                })
                .fold(
                    ((Color::Red, 0), (Color::Blue, 0), (Color::Green, 0)),
                    |(acc_red, acc_green, acc_blue), (red, green, blue)| {
                        (
                            (red.0, red.1.max(acc_red.1)),
                            (green.0, green.1.max(acc_green.1)),
                            (blue.0, blue.1.max(acc_blue.1)),
                        )
                    },
                );

            println!("{:?}", sure);

            sure.0 .1 * sure.1 .1 * sure.2 .1
        })
        .sum()
}
