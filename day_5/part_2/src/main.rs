use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let split_pos = lines.iter().position(|l| l.is_empty()).unwrap();

    let mut order_rules: HashMap<usize, Vec<usize>> = HashMap::new();

    for l in &lines[0..split_pos] {
        let pages = l
            .split('|')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        order_rules.entry(pages[0]).or_default().push(pages[1]);
    }

    lines[split_pos + 1..]
        .iter()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|u| !validate(u, &order_rules))
        .map(|mut u| {
            reorder(&mut u, &order_rules);
            u
        })
        .fold(0, |acc, u| acc + u[u.len() / 2])
}

fn validate(update: &[usize], order_rules: &HashMap<usize, Vec<usize>>) -> bool {
    for i in 1..update.len() {
        for j in 0..i {
            if let Some(rules) = order_rules.get(&update[i]) {
                if rules.contains(&update[j]) {
                    return false;
                }
            }
        }
    }

    true
}

fn reorder(update: &mut Vec<usize>, order_rules: &HashMap<usize, Vec<usize>>) {
    for i in 1..update.len() {
        for j in 0..i {
            if let Some(rules) = order_rules.get(&update[i]) {
                if rules.contains(&update[j]) {
                    update.swap(i, j);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(process(input), 123)
    }
}
