use once_cell::sync::Lazy;
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
    println!("Part 1: {}", solve(FILE, 0));
    println!("Part 2: {}", solve(FILE, 1));
}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^$").unwrap());

fn solve(input: &str, changes: usize) -> usize {
    let parts = RE.split(input).filter(|el| !el.is_empty());

    parts
        .map(|block| {
            find_vertical(block, changes)
                .unwrap_or_else(|| find_horizontal(block, changes).unwrap() * 100)
        })
        .sum()
}

fn find_vertical(block: &str, changes: usize) -> Option<usize> {
    let len = block.lines().find(|el| !el.is_empty()).unwrap().len();
    (1..len).find(|&i| {
        block
            .lines()
            .filter(|el| !el.is_empty())
            .map(|line| {
                line.bytes()
                    .skip(i)
                    .zip(line.bytes().take(i).rev())
                    .map(|(a, b)| if a == b { 0usize } else { 1 })
                    .sum::<usize>()
            })
            .sum::<usize>()
            == changes
    })
}

fn find_horizontal(block: &str, changes: usize) -> Option<usize> {
    let lines: Vec<Vec<_>> = block
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    (1..lines.len()).find(|&i| {
        let min = i.min(lines.len() - i);

        let left = lines[(i - min)..i].iter().collect::<Vec<_>>();
        let right = lines[i..(i + min)].iter().rev().collect::<Vec<_>>();

        left.iter()
            .zip(right.iter())
            .map(|(left, right)| {
                left.iter()
                    .zip(right.iter())
                    .map(|(left, right)| if left == right { 0 } else { 1 })
                    .sum::<usize>()
            })
            .sum::<usize>()
            == changes
    })
}

// fn part_2(input: &str) -> usize {
//     let parts = RE.split(input).filter(|el| !el.is_empty());
//
//     parts
//         .map(|block| {
//             let old_match =
//                 find_vertical(block).unwrap_or_else(|| find_horizontal(block).unwrap() * 100);
//             for (index, character) in block.chars().enumerate() {
//                 let inverse = if character == '#' {
//                     '.'
//                 } else if character == '.' {
//                     '#'
//                 } else {
//                     continue;
//                 };
//
//                 let new_block =
//                     block[..index].to_owned() + inverse.to_string().as_str() + &block[index + 1..];
//
//                 println!("{new_block}");
//
//                 if let Some(num) = find_vertical(&new_block) {
//                     if num != old_match {
//                         println!("vert {num}");
//                         return num;
//                     } else if let Some(num) = find_horizontal(&new_block) {
//                         if num * 100 != old_match {
//                             println!("horiz {num}");
//                             return 100 * num;
//                         }
//                     }
//                 } else if let Some(num) = find_horizontal(&new_block) {
//                     if num * 100 != old_match {
//                         println!("{new_block} horiz {num}");
//                         return 100 * num;
//                     } else if let Some(num) = find_vertical(&new_block) {
//                         if num != old_match {
//                             println!("{new_block} vert {num}");
//                             return num;
//                         }
//                     }
//                 }
//             }
//             // unreachable!();
//             0
//         })
//         .sum()
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2() {
        let input = "..##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
";
        assert_eq!(find_horizontal(input, 0), Some(3));
        assert_eq!(find_vertical(input, 0), Some(5));
    }

    #[test]
    fn test_3() {
        let input = ".##.";
        assert_eq!(find_horizontal(input, 0), None);
        assert_eq!(find_vertical(input, 0), Some(2));
    }
    #[test]
    fn test_4() {
        let input = "...#";
        assert_eq!(find_horizontal(input, 0), None);
        assert_eq!(find_vertical(input, 0), Some(1));
    }
    #[test]
    fn test_5() {
        let input = "#...";
        assert_eq!(find_horizontal(input, 0), None);
        assert_eq!(find_vertical(input, 0), Some(3));
    }

    #[test]
    fn test_6() {
        let input = ".
#
#
.";
        assert_eq!(find_horizontal(input, 0), Some(2));
        assert_eq!(find_vertical(input, 0), None);
    }
    #[test]
    fn test_7() {
        let input = ".
.
.
#";
        assert_eq!(find_horizontal(input, 0), Some(1));
        assert_eq!(find_vertical(input, 0), None);
    }

    #[test]
    fn test_8() {
        let input = "#
.
.
.";
        assert_eq!(find_horizontal(input, 0), Some(3));
        assert_eq!(find_vertical(input, 0), None);
    }

    #[test]
    fn test_9() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

        assert_eq!(find_horizontal(input, 1), Some(3));
        assert_eq!(find_vertical(input, 1), None);
    }
}
