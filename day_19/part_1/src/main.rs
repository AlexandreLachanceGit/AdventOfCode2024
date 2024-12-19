use std::collections::HashSet;

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
        .filter(|p| is_pattern_possible(&towels, p, 0, &mut HashSet::new()))
        .count()
}

fn is_pattern_possible(
    towels: &[&str],
    pattern: &str,
    done: usize,
    memo: &mut HashSet<usize>,
) -> bool {
    if done == pattern.len() {
        true
    } else {
        let p = &pattern[done..];
        if memo.contains(&done) {
            return false;
        }

        let res = towels
            .iter()
            .filter(|t| p.starts_with(**t))
            .any(|t| is_pattern_possible(towels, pattern, done + t.len(), memo));

        if !res {
            memo.insert(done);
        }
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
        assert_eq!(process(input), 6)
    }
}
