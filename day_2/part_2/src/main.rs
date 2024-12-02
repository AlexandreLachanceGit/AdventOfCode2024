// Brute force solution

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> i32 {
    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|v| {
            (0..v.len()).any(|i| {
                let candidate = v
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, &x)| x)
                    .collect::<Vec<i32>>();
                validate(candidate.iter()) || validate(candidate.iter().rev())
            })
        })
        .count() as i32
}

fn validate<'a, I>(report: I) -> bool
where
    I: Iterator<Item = &'a i32>,
{
    let mut prev: Option<&i32> = None;
    for l in report {
        if let Some(p) = prev {
            if l <= p || (l - p) > 3 {
                return false;
            }
        }
        prev = Some(l);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(process(input), 4)
    }
}
