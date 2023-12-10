use oasis::oasis_part1::part_one;

use crate::oasis::oasis_part1::part_two;

mod oasis;

fn main() {
    let first = part_one("input.data");
    let second = part_two("input.data");
    println!("First:  {}, Second: {}", first, second);
}
