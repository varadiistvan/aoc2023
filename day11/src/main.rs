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

fn part_1(input: &str) -> usize {
    // let mut matrix = input
    //     .lines()
    //     .flat_map(|line| {
    //         if line.chars().all(|ch| ch == '.') {
    //             vec![line, line]
    //         } else {
    //             vec![line]
    //         }
    //     })
    //     .map(|line| line.chars().collect::<Vec<_>>())
    //     .collect::<Vec<_>>();
    //
    // let mut i = 0;
    //
    // while i < matrix[0].len() {
    //     if matrix.iter().all(|line| line[i] == '.') {
    //         matrix.iter_mut().for_each(|line| line.insert(i, '.'));
    //         i += 1;
    //     }
    //     i += 1;
    // }
    //
    // assert!(matrix.iter().all(|line| line.len() == matrix[0].len()));
    //
    // let mut galaxies = vec![];
    //
    // matrix.iter().enumerate().for_each(|(y, line)| {
    //     line.iter().enumerate().for_each(|(x, el)| {
    //         if el == &'#' {
    //             galaxies.push((x, y))
    //         }
    //     })
    // });
    //
    // let mut sum = 0;
    //
    // for i in 0..galaxies.len() {
    //     for j in i + 1..galaxies.len() {
    //         sum += galaxies[i].0.max(galaxies[j].0) - galaxies[i].0.min(galaxies[j].0)
    //             + galaxies[i].1.max(galaxies[j].1)
    //             - galaxies[i].1.min(galaxies[j].1);
    //     }
    // }
    //
    // sum

    let mut matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut galaxies = vec![];

    matrix.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, el)| {
            if el == &'#' {
                galaxies.push((x, y))
            }
        })
    });

    let expansions_y = matrix
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|el| el == &'.'))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    let mut expansions_x = vec![];

    for i in 0..matrix[0].len() {
        if matrix.iter().all(|line| line[i] == '.') {
            expansions_x.push(i);
        }
    }

    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let crossed_x = expansions_x
                .iter()
                .filter(|coord| {
                    (galaxies[i].0.min(galaxies[j].0)..galaxies[i].0.max(galaxies[j].0))
                        .contains(coord)
                })
                .count();
            let crossed_y = expansions_y
                .iter()
                .filter(|coord| {
                    (galaxies[i].1.min(galaxies[j].1)..galaxies[i].1.max(galaxies[j].1))
                        .contains(coord)
                })
                .count();

            sum += galaxies[i].0.max(galaxies[j].0) - galaxies[i].0.min(galaxies[j].0)
                + galaxies[i].1.max(galaxies[j].1)
                - galaxies[i].1.min(galaxies[j].1)
                + (crossed_x + crossed_y);
        }
    }

    sum
}

fn part_2(input: &str) -> u128 {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut galaxies = vec![];

    matrix.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, el)| {
            if el == &'#' {
                galaxies.push((x, y))
            }
        })
    });

    let expansions_y = matrix
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|el| el == &'.'))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    let mut expansions_x = vec![];

    for i in 0..matrix[0].len() {
        if matrix.iter().all(|line| line[i] == '.') {
            expansions_x.push(i);
        }
    }

    let mut sum = 0u128;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let crossed_x = expansions_x
                .iter()
                .filter(|coord| {
                    (galaxies[i].0.min(galaxies[j].0)..galaxies[i].0.max(galaxies[j].0))
                        .contains(coord)
                })
                .count();
            let crossed_y = expansions_y
                .iter()
                .filter(|coord| {
                    (galaxies[i].1.min(galaxies[j].1)..galaxies[i].1.max(galaxies[j].1))
                        .contains(coord)
                })
                .count();

            sum += galaxies[i].0.max(galaxies[j].0) as u128
                - galaxies[i].0.min(galaxies[j].0) as u128
                + galaxies[i].1.max(galaxies[j].1) as u128
                - galaxies[i].1.min(galaxies[j].1) as u128
                + (crossed_x + crossed_y) as u128 * (100_0000u128 - 1);
        }
    }

    sum
}
