#[cfg(debug_assertions)]
const fn get_file() -> (&'static str, &'static str) {
    (include_str!("../input2.txt"), include_str!("../input4.txt"))
}

#[cfg(not(debug_assertions))]
const fn get_file() -> (&'static str, &'static str) {
    (include_str!("../input.txt"), include_str!("../input.txt"))
}

fn main() {
    const FILE: (&str, &str) = get_file();
    println!("Part 1: {}", part_1(FILE.0));
    println!("Part 2: {}", part_2(FILE.1));
}

use std::char;

fn part_1(input: &str) -> isize {
    let (srow, col) = input
        .lines()
        .enumerate()
        .find(|(_index, line)| line.contains('S'))
        .unwrap();

    let scol = col.chars().position(|el| el == 'S').unwrap();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|el| {
                    if el == 'S' {
                        (el, 'I'.to_owned(), 0)
                    } else {
                        (el, 'U'.to_owned(), -1)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if width == 5 {
        matrix[srow][scol] = ('F', 'I', 0);
    } else {
        matrix[srow][scol] = ('-', 'I', 0);
    }

    while matrix.iter().any(|line| line.iter().any(|el| el.1 == 'U')) {
        let mut changed = false;

        for row in 0..height {
            let _line = &matrix[row];
            for col in 0..width {
                let i = matrix[row][col];
                if i.1 == 'U' {
                    let mut decisions = vec![];

                    if matrix[row][col].0 == '.' {
                        changed = true;
                        matrix[row][col].1 = 'N';
                        continue;
                    }

                    if row != 0 && connects(&i.0, &matrix[row - 1][col].0, 'u') {
                        decisions.push((matrix[row - 1][col].1, matrix[row - 1][col].2))
                    }
                    if row != height - 1 && connects(&i.0, &matrix[row + 1][col].0, 'd') {
                        decisions.push((matrix[row + 1][col].1, matrix[row + 1][col].2))
                    }
                    if col != 0 && connects(&i.0, &matrix[row][col - 1].0, 'l') {
                        decisions.push((matrix[row][col - 1].1, matrix[row][col - 1].2))
                    }
                    if col != width - 1 && connects(&i.0, &matrix[row][col + 1].0, 'r') {
                        decisions.push((matrix[row][col + 1].1, matrix[row][col + 1].2))
                    }

                    if decisions.iter().any(|el| el.0 == 'I') {
                        matrix[row][col].1 = 'I'.to_owned();
                        matrix[row][col].2 = decisions.iter().find(|el| el.0 == 'I').unwrap().1 + 1;
                        changed = true;
                    } else if !decisions.is_empty() && decisions.iter().all(|el| el.0 == 'N') {
                        matrix[row][col].1 = 'N';
                        changed = true;
                    }

                    if decisions.is_empty() {
                        matrix[row][col].1 = 'N';
                        changed = true;
                    }
                }
            }
        }

        if !changed {
            matrix.iter().for_each(|line| println!("{line:?}"));

            for i in matrix.iter_mut() {
                for j in i.iter_mut() {
                    if j.1 == 'U' {
                        *j = (j.0, 'N', -1)
                    }
                }
            }
        }
    }

    let matrix: Vec<Vec<_>> = matrix
        .iter()
        .map(|line| {
            line.iter()
                .map(|el| {
                    if el.1 == 'N' {
                        (".".to_owned(), el.2)
                    } else {
                        (el.0.to_string(), el.2)
                    }
                })
                .collect()
        })
        .collect();

    matrix.iter().flatten().map(|el| el.1).max().unwrap()
}

fn connects(investigated: &char, other: &char, dir: char) -> bool {
    matches!(
        (investigated, dir, other),
        (&'|', 'u', '|' | 'F' | '7')
            | (&'|', 'd', '|' | 'L' | 'J')
            | (&'-', 'l', '-' | 'F' | 'L')
            | (&'-', 'r', '-' | 'J' | '7')
            | (&'L', 'u', '|' | 'F' | '7')
            | (&'L', 'r', '-' | 'J' | '7')
            | (&'J', 'u', '|' | 'F' | '7')
            | (&'J', 'l', '-' | 'F' | 'L')
            | (&'7', 'l', '-' | 'F' | 'L')
            | (&'7', 'd', '|' | 'L' | 'J')
            | (&'F', 'r', '-' | 'J' | '7')
            | (&'F', 'd', '|' | 'L' | 'J')
    )
}

fn part_2(input: &str) -> isize {
    println!("starting 2");

    let (srow, col) = input
        .lines()
        .enumerate()
        .find(|(_index, line)| line.contains('S'))
        .unwrap();

    let scol = col.chars().position(|el| el == 'S').unwrap();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|el| {
                    if el == 'S' {
                        (el, 'I'.to_owned(), 0)
                    } else {
                        (el, 'U'.to_owned(), -1)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if width == 11 || width == 20 || width == 10 {
        matrix[srow][scol] = ('F', 'I', 0);
    } else {
        matrix[srow][scol] = ('-', 'I', 0);
    }

    while matrix.iter().any(|line| line.iter().any(|el| el.1 == 'U')) {
        let mut changed = false;

        for row in 0..height {
            let _line = &matrix[row];
            for col in 0..width {
                let i = matrix[row][col];
                if i.1 == 'U' {
                    let mut decisions = vec![];

                    if matrix[row][col].0 == '.' {
                        changed = true;
                        matrix[row][col].1 = 'N';
                        continue;
                    }

                    if row != 0 && connects(&i.0, &matrix[row - 1][col].0, 'u') {
                        decisions.push((matrix[row - 1][col].1, matrix[row - 1][col].2))
                    }
                    if row != height - 1 && connects(&i.0, &matrix[row + 1][col].0, 'd') {
                        decisions.push((matrix[row + 1][col].1, matrix[row + 1][col].2))
                    }
                    if col != 0 && connects(&i.0, &matrix[row][col - 1].0, 'l') {
                        decisions.push((matrix[row][col - 1].1, matrix[row][col - 1].2))
                    }
                    if col != width - 1 && connects(&i.0, &matrix[row][col + 1].0, 'r') {
                        decisions.push((matrix[row][col + 1].1, matrix[row][col + 1].2))
                    }

                    if decisions.iter().any(|el| el.0 == 'I') {
                        matrix[row][col].1 = 'I'.to_owned();
                        matrix[row][col].2 = decisions.iter().find(|el| el.0 == 'I').unwrap().1 + 1;
                        changed = true;
                    } else if !decisions.is_empty() && decisions.iter().all(|el| el.0 == 'N') {
                        matrix[row][col].1 = 'N';
                        changed = true;
                    }

                    if decisions.is_empty() {
                        matrix[row][col].1 = 'N';
                        changed = true;
                    }
                }
            }
        }

        if !changed {
            for i in matrix.iter_mut() {
                for j in i.iter_mut() {
                    if j.1 == 'U' {
                        *j = (j.0, 'N', -1)
                    }
                }
            }
        }
    }

    let matrix: Vec<Vec<_>> = matrix
        .iter()
        .map(|line| {
            line.iter()
                .map(|el| {
                    if el.1 == 'N' {
                        (".".to_owned(), el.2)
                    } else {
                        (el.0.to_string(), el.2)
                    }
                })
                .collect()
        })
        .collect();

    let mut matrix = matrix
        .into_iter()
        .flat_map(|el| {
            [
                el.iter()
                    .flat_map(|el| match el.0.as_str() {
                        "." => [(".", 'U'), ("#", 'U')],
                        "|" => [("|", 'U'), ("#", 'U')],
                        "-" => [("-", 'U'), ("-", 'U')],
                        "L" => [("L", 'U'), ("-", 'U')],
                        "J" => [("J", 'U'), ("#", 'U')],
                        "7" => [("7", 'U'), ("#", 'U')],
                        "F" => [("F", 'U'), ("-", 'U')],
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>(),
                el.iter()
                    .flat_map(|el| match el.0.as_str() {
                        "." => [("#", 'U'), ("#", 'U')],
                        "|" => [("|", 'U'), ("#", 'U')],
                        "-" => [("#", 'U'), ("#", 'U')],
                        "L" => [("#", 'U'), ("#", 'U')],
                        "J" => [("#", 'U'), ("#", 'U')],
                        "7" => [("|", 'U'), ("#", 'U')],
                        "F" => [("|", 'U'), ("#", 'U')],
                        _ => unreachable!(),
                    })
                    .collect(),
            ]
        })
        .collect::<Vec<_>>();

    // for i in 0..height {
    //     for j in 0..width {
    //         if matrix[i][j] == "."
    //             && matrix[i]
    //                 .iter()
    //                 .take(j)
    //                 .filter(|el| ["|", "F", "7"].contains(el))
    //                 .count()
    //                 % 2
    //                 == 1
    //         {
    //             count += 1
    //         }
    //     }
    // }

    let len = matrix.len();

    matrix[0].iter_mut().filter(|el| el.0 == ".").for_each(|el| el.1 = 'N');
    matrix[len - 1].iter_mut().filter(|el| el.0 == ".").for_each(|el| el.1 = 'N');

    while matrix.iter().any(|line| {
        line.iter()
            .any(|el| (el.0 == "#" || el.0 == ".") && el.1 == 'U')
    }) {
        let mut changed = false;

        for i in 0..matrix.len() {
            let line = matrix[i].clone();
            for j in 0..matrix[0].len() {
                let elem = matrix[i][j];

                if elem.1 == 'U' && (elem.0 == "." || elem.0 == "#") {
                    if i != 0
                        && matrix[i - 1][j].1 == 'N'
                        && (matrix[i - 1][j].0 == "." || matrix[i - 1][j].0 == "#")
                    {
                        matrix[i][j].1 = 'N';
                        changed = true;
                    }

                    if i != matrix.len() - 1
                        && matrix[i + 1][j].1 == 'N'
                        && (matrix[i + 1][j].0 == "." || matrix[i + 1][j].0 == "#")
                    {
                        matrix[i][j].1 = 'N';
                        changed = true
                    }

                    if j != 0
                        && matrix[i][j - 1].1 == 'N'
                        && (matrix[i][j - 1].0 == "." || matrix[i][j - 1].0 == "#")
                    {
                        matrix[i][j].1 = 'N';
                        changed = true
                    }

                    if j != matrix[0].len() - 1
                        && matrix[i][j + 1].1 == 'N'
                        && (matrix[i][j + 1].0 == "." || matrix[i][j + 1].0 == "#")
                    {
                        matrix[i][j].1 = 'N';
                        changed = true
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    let matrix: Vec<_> = matrix.iter().enumerate().filter(|(y, _)| y%2 == 0).map(|(y, line)| line.iter().enumerate().filter(|(x, el)| x%2 == 0).map(|(_, el)| el).collect::<Vec<_>>()).collect();

    let matrix = matrix
    .iter()
    .map(|line| {
        line.iter()
            .map(|el| {
                if el.0 == "." && el.1 == 'N' {
                    "#"
                } else {
                    el.0
                }
            })
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    matrix.iter().for_each(|el| println!("{}", el.iter().map(|el| el.to_owned()).collect::<String>()));


    matrix
        .iter()
        .map(|line| line.iter().filter(|el| el == &&".").count() as isize)
        .sum::<isize>()
}
