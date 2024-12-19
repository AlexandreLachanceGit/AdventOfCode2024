use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> usize {
    let mut sections = input.split("\n\n");

    let towels = sections.next().unwrap().split(", ").collect::<Vec<&str>>();
    let patterns = sections.next().unwrap().lines().collect::<Vec<&str>>();

    patterns
        .iter()
        .map(|p| count_possible_patterns(&towels, p, 0, &mut HashMap::new()))
        .sum()
}

fn count_possible_patterns(
    towels: &[&str],
    pattern: &str,
    done: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if done == pattern.len() {
        1
    } else {
        let p = &pattern[done..];
        if let Some(cost) = memo.get(&done) {
            return *cost;
        }

        let res: usize = towels
            .iter()
            .filter(|t| p.starts_with(**t))
            .map(|t| count_possible_patterns(towels, pattern, done + t.len(), memo))
            .sum();

        memo.insert(done, res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;
        assert_eq!(process(input), 16)
    }
}
