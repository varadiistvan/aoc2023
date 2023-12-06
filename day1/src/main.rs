use fancy_regex::Regex;
use once_cell::sync::Lazy;

fn main() {
    let file = include_str!("../input.txt");

    println!("{}", part_1(file));

    println!("{}", part_2(file));
}

fn part_1(file: &str) -> u32 {
    let mut sum = 0;

    for line in file.lines() {
        println!("{:?}", line.chars().collect::<Vec<_>>());
        let num_1 = line
            .chars()
            .find(|el| el.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let num_2 = line
            .chars()
            .rev()
            .find(|el| el.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap();

        sum += 10 * num_1 + num_2;
    }

    sum
}

fn part_2(file: &str) -> u32 {
    static REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?=((\d)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)))")
            .unwrap()
    });

    const NUMBERS: [(&str, u32); 18] = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    file.lines()
        .map(|line| {
            println!("{}", line);

            let num1 = unsafe {
                let temp = REGEX.captures_iter(line).next().unwrap().unwrap();

                let num = temp
                    .iter()
                    .filter(Option::is_some)
                    .last()
                    .unwrap()
                    .unwrap()
                    .as_str();

                NUMBERS.iter().find(|el| el.0 == num).unwrap().1
            };

            let num2 = unsafe {
                let temp = REGEX.captures_iter(line).last().unwrap().unwrap();

                let num = temp
                    .iter()
                    .filter(Option::is_some)
                    .last()
                    .unwrap()
                    .unwrap()
                    .as_str();

                NUMBERS.iter().find(|el| el.0 == num).unwrap().1
            };

            println!("{}, {}", num1, num2);

            10 * num1 + num2
        })
        .sum()
}
