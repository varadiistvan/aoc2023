use std::{
    collections::{hash_map::Entry, HashMap},
    hint::unreachable_unchecked,
};

#[cfg(debug_assertions)]
const fn get_file() -> &'static str {
    include_str!("../input2.txt")
}

#[cfg(not(debug_assertions))]
const fn get_file() -> &'static str {
    include_str!("../input.txt")
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn roll(map: &mut [Vec<char>], direction: Direction) {
    match direction {
        Direction::East => east(map),
        Direction::West => west(map),
        Direction::North => north(map),
        Direction::South => south(map),
    };
}

fn west(map: &mut [Vec<char>]) {
    let len = map[0].len();

    for line in map.iter_mut() {
        let mut last_cube = None;
        let mut num_round = 0;
        for i in (0..len).rev() {
            match line[i] {
                '.' => continue,
                'O' => num_round += 1,
                '#' => {
                    for j in (i + 1..=last_cube.map_or(len - 1, |val| val - 1)).rev() {
                        line[j] = if i + num_round < j { '.' } else { 'O' };
                    }
                    num_round = 0;
                    last_cube = Some(i)
                }
                _ => unsafe { unreachable_unchecked() },
            }
        }

        if num_round > 0 {
            for j in (0..=last_cube.map_or(len - 1, |val| val - 1)).rev() {
                line[j] = if num_round - 1 < j { '.' } else { 'O' };
            }
        }
    }
}

fn east(map: &mut [Vec<char>]) {
    let len = map[0].len();

    for line in map.iter_mut() {
        let mut last_cube = None;
        let mut num_round = 0;
        for i in 0..len {
            match line[i] {
                '.' => continue,
                'O' => num_round += 1,
                '#' => {
                    line.iter_mut()
                        .enumerate()
                        .take(i)
                        .skip(last_cube.map_or(0, |val| val + 1))
                        .for_each(|(j, elem)| {
                            *elem = if i - num_round > j { '.' } else { 'O' };
                        });
                    // println!("{}", ["-"; 50].join(""));
                    num_round = 0;
                    last_cube = Some(i)
                }
                _ => unsafe { unreachable_unchecked() },
            }
        }

        if num_round > 0 {
            // println!("{:?}", (last_cube.map_or(len - 1, |val| val - 1)..i).rev());
            line.iter_mut()
                .enumerate()
                .take(len)
                .skip(last_cube.map_or(0, |val| val + 1))
                .for_each(|(j, elem)| {
                    *elem = if len.saturating_sub(num_round) > j {
                        '.'
                    } else {
                        'O'
                    }
                });
        }
    }
}

fn south(map: &mut [Vec<char>]) {
    let height = map.len();
    let width = map[0].len();

    for col_index in 0..width {
        let mut last_cube = None;
        let mut num_round = 0;
        for j in 0..height {
            match map[j][col_index] {
                '.' => continue,
                'O' => num_round += 1,
                '#' => {
                    for i in last_cube.map_or(0, |val| val + 1)..j {
                        map[i][col_index] = if i + num_round < j { '.' } else { 'O' };
                    }

                    num_round = 0;
                    last_cube = Some(j)
                }
                _ => unsafe { unreachable_unchecked() },
            }
        }

        if num_round > 0 {
            for i in last_cube.map_or(0, |val| val + 1)..height {
                map[i][col_index] = if i + num_round < height { '.' } else { 'O' };
            }
        }
    }
}

fn north(map: &mut [Vec<char>]) {
    let width = map[0].len();
    let height = map.len();

    for col_index in 0..width {
        let mut last_cube = None;
        let mut num_round = 0;
        for j in (0..height).rev() {
            match map[j][col_index] {
                '.' => continue,
                'O' => num_round += 1,
                '#' => {
                    for i in (j + 1)..last_cube.unwrap_or(height) {
                        map[i][col_index] = if i.saturating_sub(num_round) > j {
                            '.'
                        } else {
                            'O'
                        };
                    }

                    num_round = 0;
                    last_cube = Some(j)
                }
                _ => unsafe { unreachable_unchecked() },
            }
        }

        if num_round > 0 {
            for i in 0..last_cube.unwrap_or(height) {
                map[i][col_index] = if num_round - 1 < i { '.' } else { 'O' };
            }
        }
    }
}
fn cycle(map: &mut [Vec<char>]) {
    roll(map, Direction::North);
    roll(map, Direction::West);
    roll(map, Direction::South);
    roll(map, Direction::East);
}

fn main() {
    const FILE: &str = get_file();
    println!("Part 1: {}", part_1(FILE));
    println!("Part 2: {}", part_2(FILE));
}

fn part_1(file: &str) -> usize {
    let mut map: Vec<Vec<char>> = file.lines().map(|l| l.chars().collect()).collect();

    map.iter()
        .for_each(|l| println!("{}", l.iter().collect::<String>()));

    roll(&mut map, Direction::North);

    map.into_iter()
        .rev()
        .enumerate()
        .map(|(index, line)| line.iter().filter(|el| **el == 'O').count() * (index + 1))
        .sum()
}

fn part_2(file: &str) -> usize {
    let mut map = file
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let mut visited: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    let mut loop_vals: (usize, usize) = (0, 0);

    let mut test_map = map.clone();

    for i in 0.. {
        match visited.entry(test_map.clone()) {
            Entry::Occupied(start) => {
                loop_vals = (*start.get(), i);
                break;
            }
            Entry::Vacant(entry) => {
                entry.insert(i);
                cycle(&mut test_map);
            }
        }
    }

    let iters = (1_000_000_000 - loop_vals.0) % (loop_vals.1 - loop_vals.0) + loop_vals.0;
    for _ in 0..iters {
        cycle(&mut map);
    }

    map.into_iter()
        .rev()
        .enumerate()
        .map(|(index, line)| line.iter().filter(|el| **el == 'O').count() * (index + 1))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{roll, Direction};

    #[test]
    fn test1() {
        let input = "##.O###.#.##
OO...O.O....
...#..O.O#..
.O...O.#OOOO
.##.O......O
O.OOO#O...O.
#.O......O.#
....#..O.O..
#...#.....#.
...O##OO.O##
...O...O....
.....#....#.
OO..#O#..##O
..O..OO.###.
.#..##.O.O#.
O..OO.#OO#..
...#........
O.....O#....
.OO.#.#.....
O.OO#.......
.#O.O..O#...
..OO#.O....O
.#.O#.#.O#..
OO....OO..O#
..........OO
...#.#..#.#.
......O#...O
..#....O.O#.
....#....#O.
..........O#
......O...O.
O#O.O....#.O
.....O.O..O.
..OO.O.O#...
.O.#...O....
#..##.#O#.#.
...#..O...O.
.O.O.#O.O#..
.OO.O...O##.
.#.#O.OOOO..
O..#..#.....
......OO....
...OO...#..#
#..#.#O.O.#.
O.##O...O..O
O.O#....#.#.
.O#.#.......
OOO..#.O.O.O
#OO.#....##.
..O......##.
.O.OO...O...
........O..O
..O####.#.#.
....##..O.O#
..##....O...
.#O...O.O...
..O..#OOO#.O
..#...O....#
..#.O...#O.#
....O#O.##.#
.O#...##.O..
.....OO..#O.
.#..O#.#....
O....O..O.#.
.O......#..#
O..#.#.#....
OO.O####.O..
...#....O.#.
O#..OO......
#....O.O...#
.OO.#.OO....
O..OOO...#..
.OO.O#...#O.
OO#..O..##..
.#.O.....###
.....O.#....
.#.O..O.O.#.
OO..#.O..OO.
.O...#.O.#OO
..O.#....#.O
O.O...##.#OO
........O...
.#....O.#...
OO....#O.O..
#.....#..#O.
..#.#..#...#
...O.#.O....
O#....###.OO
#.O...O#..O.
..#O#O..#O.O
OO#..OOO#..#
.#..O.#O...O
#.O....#.#.O
.#.O.OOO...O
.#O...OO...O
.O#.OOO.....
..#.O..O..O.
.O...#..O...
.##.#..O.O..
#O..O.......";

        assert!(true);
    }

    #[test]
    fn test2() {
        let input = "#
O
.
.
.
O
#
.
#
.
.
.
O
.
.
O
.
O
.
O
.
.
.
O
.
.
.
.
.
.
.
O
.
.
.
#
.
.
.
.
O
.
.
#
O
O
.
O
#
.
.
.
.
.
.
.
.
.
.
.
.
.
.
O
.
O
O
.
O
#
.
O
.
O
.
.
.
O
.
.
O
.
.
O
#
.
.
O
#
.
O
.
#
.
.
.
.
.
.
#";

        let out = "#
O
O
.
.
.
#
.
#
O
O
O
O
O
O
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
#
O
.
.
.
.
.
.
#
O
O
O
.
#
O
O
O
O
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
.
#
O
O
O
O
O
.
.
.
.
.
.
.
.
.
#
O
.
.
#
O
.
.
#
.
.
.
.
.
.
#";
        let mut in_bruh = input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<_>>();

        roll(&mut in_bruh, Direction::North);

        assert_eq!(
            out.lines()
                .map(|l| l.chars().collect())
                .collect::<Vec<Vec<_>>>(),
            in_bruh
        );
    }
}
