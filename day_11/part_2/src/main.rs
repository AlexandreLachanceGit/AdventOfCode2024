// only change 25 -> 75

use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> u64 {
    let mut rocks = input
        .split_whitespace()
        .map(|n| (n.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<u64, u64>>();

    for _ in 0..75 {
        rocks = blink(&rocks);
    }

    rocks.values().sum()
}

fn blink(rocks: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_rocks = HashMap::new();

    for r in rocks.keys() {
        if *r == 0 {
            *new_rocks.entry(1).or_default() += rocks[r];
        } else if nb_digits(*r) % 2 == 0 {
            let (a, b) = split_nb(*r);
            *new_rocks.entry(a).or_default() += rocks[r];
            *new_rocks.entry(b).or_default() += rocks[r];
        } else {
            *new_rocks.entry(r * 2024).or_default() += rocks[r];
        }
    }

    new_rocks
}

fn nb_digits(number: u64) -> u32 {
    number.to_string().len() as u32
}

fn split_nb(number: u64) -> (u64, u64) {
    let mult = 10_u64.pow(nb_digits(number) / 2);
    (number / mult, number % mult)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"125 17"#;
        assert_eq!(process(input), 55312)
    }

    #[test]
    fn test_nb_digits() {
        assert_eq!(6, nb_digits(123456))
    }

    #[test]
    fn test_split_nb() {
        assert_eq!((123, 456), split_nb(123456))
    }
}
