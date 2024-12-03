use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let mut total = 0;

    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(process(input), 161)
    }
}
