use std::{collections::HashMap, rc::Rc, sync::Arc};

use once_cell::sync::Lazy;
use regex::Regex;
use scoped_threadpool::Pool;

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

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\(.+\)").unwrap());

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().chars().cycle();

    let lines = input.lines().skip(2).collect::<Vec<_>>();

    let mut length = 0;
    let mut curr = Rc::new("AAA".to_owned());

    let mut cache = HashMap::new();

    for i in instructions {
        let lines = lines.clone();
        let curr_loop = curr.clone();
        let paths = cache.entry(curr).or_insert_with(|| {
            let found = lines
                .iter()
                .find(|el| el.split_whitespace().next().unwrap() == *curr_loop)
                .map(|el| {
                    let mut parts = RE.find(el).unwrap().as_str().split(", ");
                    let left = parts.next().unwrap()[1..].to_owned();
                    let right = &parts.next().unwrap();
                    let right = right[..right.len() - 1].to_owned();
                    (left, right)
                });
            found.unwrap()
        });

        match i {
            'L' => curr = Rc::new(paths.0.clone()),
            'R' => curr = Rc::new(paths.1.clone()),
            _ => unreachable!(),
        }

        length += 1;

        if *curr == "ZZZ" {
            break;
        }
    }

    length
}

fn part_2(input: &str) -> u128 {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().chars().cycle();

    let lines = input.lines().skip(2).collect::<Vec<_>>();

    let len = input.lines().next().unwrap().chars().count();

    let starters = lines
        .iter()
        .filter(|el| el.split_whitespace().next().unwrap().ends_with('A'))
        .map(|el| el.split_whitespace().next().unwrap().to_owned())
        .collect::<Vec<String>>();

    let mut cache = HashMap::new();

    let loops: Vec<_> = starters
        .into_iter()
        .map(|starter| {
            let mut this_loop: Vec<(String, u128)> = vec![(starter.clone(), 0)];

            let mut curr = Rc::new(starter.clone());

            let mut loop_start = 0;

            for (index, i) in instructions.clone().enumerate() {
                let lines = lines.clone();
                let curr_loop = curr.clone();
                let paths = cache.entry(curr.clone()).or_insert_with(|| {
                    let found = lines
                        .iter()
                        .find(|el| el.split_whitespace().next().unwrap() == *curr_loop)
                        .map(|el| {
                            let mut parts = RE.find(el).unwrap().as_str().split(", ");
                            let left = parts.next().unwrap()[1..].to_owned();
                            let right = &parts.next().unwrap();
                            let right = right[..right.len() - 1].to_owned();
                            (left, right)
                        });
                    found.unwrap()
                });

                let new = match i {
                    'L' => paths.0.clone(),
                    'R' => paths.1.clone(),
                    _ => unreachable!(),
                };

                curr = Rc::new(new.clone());

                let elem = (new.clone(), ((index + 1) % len) as u128);

                if this_loop.contains(&elem) {
                    loop_start = this_loop.iter().position(|el| el == &elem).unwrap();
                    break;
                } else {
                    this_loop.push(elem);
                }
            }
            (
                this_loop.len() as u128,
                this_loop
                    .iter()
                    .enumerate()
                    .filter(|el| el.1 .0.ends_with('Z'))
                    .map(|el| (el.0 - loop_start) as u128)
                    .collect::<Vec<_>>(),
                loop_start as u128,
            )
        })
        .collect();

    // let found = Arc::new(crossbeam::atomic::AtomicCell::new(0u128));

    // let mut pool = Pool::new(16);

    // let arc_loops = Arc::new(loops);

    // pool.scoped(|scope| {
    //     for i in 0u128.. {
    //         if i % 1_000_000_000 == 0 {
    //             println!("{}", i);
    //         }

    //         if found.load() != 0 {
    //             break
    //         }

    //         let my_loops = arc_loops.clone();
    //         let my_found = found.clone();

    //         scope.execute(move || {
    //             if my_loops.iter().all(|el| {

    //                 i >= el.2 && el.1.contains(&((i - el.2) % (el.0 - el.2)))

    //             }) {
    //                 println!("{i}");
    //                 my_found.compare_exchange(0, i);
    //             }
    //         })

    //     }
    // });

    println!("{loops:?}");


    let mut lcm = 1u128;
    for (total_length, points_of_interest, start_of_loop) in loops {
        let loop_size = total_length - start_of_loop;
        for point in points_of_interest {
            let mut step = loop_size;
            while step < point {
                step += loop_size;
            }
            lcm = lcm / gcd(lcm, step) * step;
        }
    }

    lcm

}
fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 { a } else { gcd(b, a % b) }
}
