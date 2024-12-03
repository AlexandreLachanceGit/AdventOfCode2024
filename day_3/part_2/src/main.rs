use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d*),(\d*)\)|()()do\(\)|()()don't\(\)").unwrap();

    let mut total = 0;
    let mut do_next = true;

    for (str, caps) in re.captures_iter(input).map(|c| c.extract()) {
        match str {
            "do()" => do_next = true,
            "don't()" => do_next = false,
            _ => {
                if do_next {
                    let [a, b] = caps;
                    total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(process(input), 48)
    }
}
