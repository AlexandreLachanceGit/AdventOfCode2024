use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

const COST_A: i128 = 3;
const COST_B: i128 = 1;
const OFFSET: i128 = 10000000000000;

#[derive(Debug)]
struct Machine {
    button_a: (i128, i128),
    button_b: (i128, i128),
    prize: (i128, i128),
}

impl Machine {
    fn find_cheapest(&self) -> i128 {
        // 8400 = 94a + 22b
        // 5400 = 34a + 67b
        // 5400 * 22 = 22 * 34a + 67 * (8400 - 94a)
        // 5400 * 22 = 22 * 34a + 67 * 8400 - 67 * 94a
        // 5400 * 22 - 67 * 8400 = 22 * 34a - 67 * 94a
        // (5400 * 22 - 67 * 8400) / (22 * 34 - 67 * 94) = a

        let a = (self.prize.1 * self.button_b.0 - self.button_b.1 * self.prize.0)
            / (self.button_b.0 * self.button_a.1 - self.button_b.1 * self.button_a.0);
        let b = (self.prize.0 - self.button_a.0 * a) / self.button_b.0;

        if self.prize.0 != self.button_a.0 * a + self.button_b.0 * b
            || self.prize.1 != self.button_a.1 * a + self.button_b.1 * b
            || a < 0
            || b < 0
        {
            0
        } else {
            COST_A * a + COST_B * b
        }
    }
}

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> i128 {
    parse_machines(input)
        .iter()
        // .inspect(|m| println!("{m:?}"))
        .map(|m| m.find_cheapest())
        .sum()
}

fn parse_machines(input: &str) -> Vec<Machine> {
    let re = Regex::new(
        r"Button A: X\+(\d*), Y\+(\d*)\nButton B: X\+(\d*), Y\+(\d*)\nPrize: X=(\d*), Y=(\d*)\n",
    )
    .unwrap();

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, arr)| arr.map(|x| x.parse::<i128>().unwrap()))
        .map(|[a_x, a_y, b_x, b_y, prize_x, prize_y]| Machine {
            button_a: (a_x, a_y),
            button_b: (b_x, b_y),
            prize: (prize_x + OFFSET, prize_y + OFFSET),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;
        assert_eq!(process(input), 480)
    }
}
