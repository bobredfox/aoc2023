use std::env;
use std::fs;

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn new(string: &str) -> Card {
        return Card {
            id: string
                .split_whitespace()
                .skip(1)
                .take(1)
                .map(|x| {
                    x.strip_suffix(":")
                        .unwrap()
                        .to_owned()
                        .parse::<usize>()
                        .unwrap()
                })
                .sum(),
            winning_numbers: string
                .split_whitespace()
                .skip(2)
                .take(10)
                .map(|x| x.to_owned().parse::<usize>().expect("Parsing went wrong!"))
                .collect(),
            numbers: string
                .split_whitespace()
                .skip(13)
                .map(|x| x.to_owned().parse::<usize>().expect("Second parsing!"))
                .collect(),
        };
    }

    fn score(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|w| self.numbers.contains(w))
            .fold(0, |acc, x| match acc {
                0 => 1,
                _ => acc + 1,
            })
    }
}

fn read_input(path: &str) -> Vec<Card> {
    let string = fs::read_to_string(path).expect("File not found!");
    let card_strings: Vec<&str> = string.lines().collect();
    let cards: Vec<Card> = card_strings.iter().map(|c| Card::new(c)).collect();
    return cards;
}

fn main() {
    let cards = read_input("input.data");
    let sum: usize = cards.iter().map(|c| c.score()).sum();
    println!("{:?}", cards);
    println!("Sum is: {}", sum);
}
