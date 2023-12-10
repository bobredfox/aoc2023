use camelcards::camel_cards::{Cards, Hand, Types};
use std::fs;

mod camelcards;

fn read_input(str_path: &str) -> Vec<Hand> {
    let string = fs::read_to_string(str_path).unwrap();
    let lines: Vec<&str> = string.lines().collect();
    return lines.iter().map(|x| split_line(x)).collect();
}

fn split_line(line: &str) -> Hand {
    return Hand::new(
        line.split_whitespace().take(1).last().unwrap(),
        line.split_whitespace()
            .last()
            .unwrap()
            .to_owned()
            .parse::<usize>()
            .unwrap(),
    );
}

fn main() {
    let hands = read_input("input.data");
    println!("{:?}", hands);
}
