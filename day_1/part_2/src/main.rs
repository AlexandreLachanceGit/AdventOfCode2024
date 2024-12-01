use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> i32 {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    input
        .lines()
        .map(|s| s.split_whitespace().map(|n| n.parse::<i32>().unwrap()))
        .for_each(|mut l| {
            left.push(l.next().unwrap());
            right.push(l.next().unwrap());
        });

    let mut counts: HashMap<i32, i32> = HashMap::new();
    right
        .into_iter()
        .for_each(|n| *counts.entry(n).or_insert(0) += 1);

    left.into_iter()
        .fold(0, |acc, x| acc + x * counts.get(&x).unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(process(input), 31)
    }
}
