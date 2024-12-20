use core::panic;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Default)]
struct State {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    pointer: usize,
    out: Vec<u8>,
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

fn run_program(state: &mut State, instructions: &[Instruction]) -> Vec<u8> {
    while state.pointer < instructions.len() {
        // if state.pointer == 0 {
        //     println!("{:o} {:?}", state.reg_a, state.out);
        // }

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
    // println!("{:o} {:?}", state.reg_a, state.out);
    state.out.clone()
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.split(':')
        .last()
        .unwrap()
        .split(',')
        .map(|s| s.trim().parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>()
}

fn find_reg_a(instructions: &[Instruction]) -> u64 {
    let instructions_octal = instructions
        .iter()
        .map(|i| i.literal() as u8)
        .collect::<Vec<u8>>();

    let mut reg_a_octal: Vec<u8> = vec![0; instructions.len()];
    reg_a_octal[0] = 1;

    let mut pos = 0;
    let mut result = vec![];
    while instructions_octal != result {
        if result[result.len() - pos..] == instructions_octal[result.len() - pos..] {
            pos += 1;
        } else if pos == instructions.len() {
            pos -= 1;
            while reg_a_octal[pos] == 7 {
                reg_a_octal[pos] = 0;
                pos -= 1;
            }
            reg_a_octal[pos] += 1;
        } else if reg_a_octal[pos] < 7 {
            reg_a_octal[pos] += 1;
        } else {
            while reg_a_octal[pos] == 7 {
                reg_a_octal[pos] = 0;
                pos -= 1;
            }
            reg_a_octal[pos] += 1;
        }
        result = run_program(
            &mut State {
                reg_a: octal_to_dec(&reg_a_octal),
                ..Default::default()
            },
            instructions,
        );
    }

    octal_to_dec(&reg_a_octal)
}

fn octal_to_dec(octal: &[u8]) -> u64 {
    octal
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, o)| acc + *o as u64 * 8_u64.pow(i as u32))
}

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> u64 {
    let instructions_str = input.split("\n\n").last().unwrap();
    let instructions = parse_instructions(instructions_str);

    find_reg_a(&instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_find_reg_a() {
        let instructions_str = INPUT.split("\n\n").last().unwrap();
        let instructions = parse_instructions(instructions_str);
        let state = &mut State {
            reg_a: find_reg_a(&instructions),
            ..Default::default()
        };

        run_program(state, &instructions);

        assert!(
            instructions
                .iter()
                .map(|i| i.literal() as u8)
                .collect::<Vec<u8>>()
                == state.out
        )
    }

    #[test]
    fn test_octal_to_dec() {
        assert_eq!(octal_to_dec(&[6, 2, 7]), 407)
    }

    #[test]
    fn test0() {
        let mut state = State {
            reg_a: 0,
            reg_b: 0,
            reg_c: 9,
            pointer: 0,
            out: Vec::new(),
        };

        run_program(&mut state, &parse_instructions("2,6"));

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

        run_program(&mut state, &parse_instructions("5,0,5,1,5,4"));

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

        run_program(&mut state, &parse_instructions("0,1,5,4,3,0"));

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

        run_program(&mut state, &parse_instructions("1,7"));

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

        run_program(&mut state, &parse_instructions("4,0"));

        assert_eq!(state.reg_b, 44354);
    }

    #[test]
    fn example() {
        let input = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
        assert_eq!(process(input), 117440)
    }
}
