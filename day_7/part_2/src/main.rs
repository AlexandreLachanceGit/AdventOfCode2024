use std::{num::ParseIntError, str::FromStr};

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug)]
enum Operations {
    Add,
    Mult,
    Concat,
}

impl Operations {
    const VALUES: [Self; 3] = [Self::Add, Self::Mult, Self::Concat];

    fn exec(&self, a: usize, b: usize) -> usize {
        match *self {
            Operations::Add => a + b,
            Operations::Mult => a * b,
            Operations::Concat => {
                let b_nb_digits: u32 = f64::from(b as u32).log10() as u32 + 1;
                a * 10usize.pow(b_nb_digits) + b
            }
        }
    }

    fn get_possibilites(length: usize) -> Vec<Vec<Operations>> {
        fn helper(possibilities: Vec<Vec<Operations>>, length: usize) -> Vec<Vec<Operations>> {
            if let Some(first) = possibilities.first() {
                if first.len() == length {
                    return possibilities;
                }
            }

            let mut new_possibilities = vec![];
            for pos in possibilities.iter() {
                for op in Operations::VALUES.iter() {
                    let mut new_pos = pos.clone();
                    new_pos.push(*op);
                    new_possibilities.push(new_pos);
                }
            }

            helper(new_possibilities, length)
        }

        helper(
            Operations::VALUES.iter().map(|op| vec![*op]).collect(),
            length,
        )
    }
}

struct Line {
    result: usize,
    numbers: Vec<usize>,
}

#[derive(Debug)]
enum ParseError {
    ParseInt(ParseIntError),
    WrongStructure,
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::ParseInt(err)
    }
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(':').collect();
        if split.len() != 2 {
            return Err(ParseError::WrongStructure);
        }

        let result = split[0].parse::<usize>()?;
        let numbers = split[1]
            .split_whitespace()
            .map(|n| n.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Line { result, numbers })
    }
}

impl Line {
    fn validate(&self) -> bool {
        let op_possibilites = Operations::get_possibilites(self.numbers.len() - 1);

        op_possibilites.into_iter().any(|possibilites| {
            let mut total = self.numbers[0];
            for (n, op) in self.numbers.iter().skip(1).zip(possibilites.iter()) {
                total = op.exec(total, *n);
                if total > self.result {
                    return false;
                }
            }
            total == self.result
        })
    }
}

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<Line>().unwrap())
        .filter(|l| l.validate())
        .fold(0, |acc, n| acc + n.result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        assert_eq!(process(input), 11387)
    }

    #[test]
    fn op() {
        assert_eq!(Operations::Concat.exec(12, 34), 1234)
    }
}
