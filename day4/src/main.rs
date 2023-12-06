use std::fmt::Display;

#[cfg(debug_assertions)]
const fn get_file() -> (&'static str, &'static str) {
    (include_str!("../input2.txt"), include_str!("../input3.txt"))
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    playing: Vec<usize>,
    winners: Vec<usize>,
}

impl From<(&str, &str)> for Card {
    fn from(value: (&str, &str)) -> Self {
        Self {
            playing: value
                .1
                .split_whitespace()
                .map(|el| el.parse::<usize>().unwrap())
                .collect(),
            winners: value
                .0
                .split_whitespace()
                .map(|el| el.parse::<usize>().unwrap())
                .collect(),
        }
    }
}

impl Card {
    fn get_win_count(self) -> usize {
        self.playing
            .iter()
            .filter(|el| self.winners.contains(el))
            .count()
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "playing: {:?}, winners: {:?}",
            self.playing, self.winners
        ))
    }
}

fn part_1(input: &str) -> impl Display {
    input
        .lines()
        .filter_map(|el| {
            let mut relevant = el.split(':').last()?.split('|');
            Some((relevant.next()?, relevant.next()?))
        })
        .map(Card::from)
        .map(Card::get_win_count)
        .map(|el| if el > 0 { 2u128.pow(el as u32 - 1) } else { 0 })
        .sum::<u128>()
}

fn part_2(input: &str) -> u128 {
    let mut cards = vec![1; input.lines().count()];

    input.lines().enumerate().fold(0, |acc, (index, el)| {
        let relevant = el.split(':').last().unwrap();

        let mut split = relevant.split('|');

        let card = Card::from((split.next().unwrap(), split.next().unwrap()));

        let win_count = card.get_win_count().min(cards.len() - 1 - index);

        for i in index + 1..index + 1 + win_count {
            cards[i] += cards[index];
        }

        acc + cards[index]
    });

    cards.iter().sum::<u128>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "Card 1: 1 | 1 1 1 1 \n Card 2: 1 | 0 0 0 0";

        assert_eq!(part_2(input), 3);
    }

    #[test]
    fn test_2() {
        let input = "Card 1: 1 | 1 1 1 1 \n Card 2: 1 | 1 1 1 1";

        assert_eq!(part_2(input), 3);
    }
}
