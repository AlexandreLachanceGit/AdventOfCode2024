use core::panic;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone)]
struct State {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    pointer: usize,
    out: Vec<u8>,
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut registers = s
            .lines()
            .map(|l| l.split(':').last().unwrap().trim())
            .map(|s| s.parse::<u64>().unwrap());

        Ok(State {
            reg_a: registers.next().unwrap(),
            reg_b: registers.next().unwrap(),
            reg_c: registers.next().unwrap(),
            pointer: 0,
            out: Vec::new(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Instruction::Adv),
            "1" => Ok(Instruction::Bxl),
            "2" => Ok(Instruction::Bst),
            "3" => Ok(Instruction::Jnz),
            "4" => Ok(Instruction::Bxc),
            "5" => Ok(Instruction::Out),
            "6" => Ok(Instruction::Bdv),
            "7" => Ok(Instruction::Cdv),
            _ => Err(()),
        }
    }
}

impl Instruction {
    fn combo(&self, state: &State) -> u64 {
        match self {
            Instruction::Adv => 0,
            Instruction::Bxl => 1,
            Instruction::Bst => 2,
            Instruction::Jnz => 3,
            Instruction::Bxc => state.reg_a,
            Instruction::Out => state.reg_b,
            Instruction::Bdv => state.reg_c,
            Instruction::Cdv => panic!("should not appear in valid program"),
        }
    }

    fn literal(&self) -> u64 {
        match self {
            Instruction::Adv => 0,
            Instruction::Bxl => 1,
            Instruction::Bst => 2,
            Instruction::Jnz => 3,
            Instruction::Bxc => 4,
            Instruction::Out => 5,
            Instruction::Bdv => 6,
            Instruction::Cdv => 7,
        }
    }
}

fn run_program(state: &mut State, instructions: Vec<Instruction>) {
    while state.pointer < instructions.len() {
        let instruction = instructions[state.pointer];
        let op = instructions[state.pointer + 1];

        let mut jumped = false;
        match instruction {
            Instruction::Adv => state.reg_a /= 2u64.pow(op.combo(state) as u32),
            Instruction::Bxl => state.reg_b ^= op.literal(),
            Instruction::Bst => state.reg_b = op.combo(state) % 8,
            Instruction::Jnz => {
                if state.reg_a != 0 {
                    state.pointer = op.literal() as usize;
                    jumped = true;
                }
            }
            Instruction::Bxc => state.reg_b ^= state.reg_c,
            Instruction::Out => state.out.push((op.combo(state) % 8) as u8),
            Instruction::Bdv => state.reg_b = state.reg_a / 2u64.pow(op.combo(state) as u32),
            Instruction::Cdv => state.reg_c = state.reg_a / 2u64.pow(op.combo(state) as u32),
        }

        if !jumped {
            state.pointer += 2;
        }
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.split(':')
        .last()
        .unwrap()
        .split(',')
        .map(|s| s.trim().parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>()
}

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> String {
    let mut sections = input.split("\n\n");

    let state_str = sections.next().unwrap();
    let mut state = State::from_str(state_str).unwrap();

    let instructions_str = sections.next().unwrap();
    let instructions = parse_instructions(instructions_str);

    run_program(&mut state, instructions);

    state
        .out
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mut state = State {
            reg_a: 0,
            reg_b: 0,
            reg_c: 9,
            pointer: 0,
            out: Vec::new(),
        };

        run_program(&mut state, parse_instructions("2,6"));

        assert_eq!(state.reg_b, 1);
    }

    #[test]
    fn test1() {
        let mut state = State {
            reg_a: 10,
            reg_b: 0,
            reg_c: 0,
            pointer: 0,
            out: Vec::new(),
        };

        run_program(&mut state, parse_instructions("5,0,5,1,5,4"));

        assert_eq!(state.out, vec![0, 1, 2]);
    }

    #[test]
    fn test2() {
        let mut state = State {
            reg_a: 2024,
            reg_b: 0,
            reg_c: 0,
            pointer: 0,
            out: Vec::new(),
        };

        run_program(&mut state, parse_instructions("0,1,5,4,3,0"));

        assert_eq!(state.out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(state.reg_a, 0);
    }

    #[test]
    fn test3() {
        let mut state = State {
            reg_a: 0,
            reg_b: 29,
            reg_c: 0,
            pointer: 0,
            out: Vec::new(),
        };

        run_program(&mut state, parse_instructions("1,7"));

        assert_eq!(state.reg_b, 26);
    }

    #[test]
    fn test4() {
        let mut state = State {
            reg_a: 0,
            reg_b: 2024,
            reg_c: 43690,
            pointer: 0,
            out: Vec::new(),
        };

        run_program(&mut state, parse_instructions("4,0"));

        assert_eq!(state.reg_b, 44354);
    }

    #[test]
    fn example() {
        let input = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
        assert_eq!(process(input), "4,6,3,5,6,3,5,2,1,0".to_string())
    }
}
