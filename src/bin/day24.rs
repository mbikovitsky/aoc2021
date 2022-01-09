use std::{borrow::Borrow, str::FromStr};

use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::util::input_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Variable {
    X = 0,
    Y = 1,
    Z = 2,
    W = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operand {
    Variable(Variable),
    Immediate(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Operand),
    Mul(Variable, Operand),
    Div(Variable, Operand),
    Mod(Variable, Operand),
    Eql(Variable, Operand),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InstructionBlockType {
    Type1(i64),
    Type2(i64, i64),
}

fn main() -> Result<()> {
    let instructions = parse_input()?;

    let largest_model_number = find_model_number(&instructions, true).unwrap();
    dbg!(largest_model_number);

    let smallest_model_number = find_model_number(&instructions, false).unwrap();
    dbg!(smallest_model_number);

    Ok(())
}

fn find_model_number(instructions: &[Instruction], largest: bool) -> Option<u64> {
    fn backtrack(
        digits: &mut [i64; 14],
        index: usize,
        instructions: &[Instruction],
        largest: bool,
    ) -> Option<[i64; 14]> {
        if index == digits.len() {
            let variables = execute_alu_program(instructions, &*digits).unwrap();
            if variables[Variable::Z as usize] == 0 {
                return Some(*digits);
            } else {
                return None;
            }
        }

        let block_type =
            classify_instruction_block(&instructions[index * 18 + 1..(index + 1) * 18]);

        match block_type {
            InstructionBlockType::Type1(_) => {
                let start = if largest { 9 } else { 1 };
                let end = if largest { 0 } else { 10 };
                let step = if largest { -1 } else { 1 };

                let mut digit = start;
                while digit != end {
                    digits[index] = digit;
                    if let Some(result) = backtrack(digits, index + 1, instructions, largest) {
                        return Some(result);
                    }
                    digit += step;
                }

                None
            }
            InstructionBlockType::Type2(add1, _) => {
                let variables =
                    execute_alu_program(&instructions[..index * 18], &digits[..index]).unwrap();
                let w = variables[Variable::Z as usize] % 26 + add1;
                if 1 <= w && w <= 9 {
                    digits[index] = w;
                    backtrack(digits, index + 1, instructions, largest)
                } else {
                    None
                }
            }
        }
    }

    backtrack(&mut [0; 14], 0, instructions, largest).map(|digits| {
        let mut result = 0;
        for digit in digits {
            result *= 10;
            result += digit as u64;
        }
        result
    })
}

fn classify_instruction_block(instructions: &[Instruction]) -> InstructionBlockType {
    let div = if let Instruction::Div(Variable::Z, Operand::Immediate(imm)) = instructions[3] {
        imm
    } else {
        panic!("Fourth instruction is not div z <imm>")
    };

    let add1 = if let Instruction::Add(Variable::X, Operand::Immediate(imm)) = instructions[4] {
        imm
    } else {
        panic!("Fifth instruction is not add x <imm>")
    };

    let add2 = if let Instruction::Add(Variable::Y, Operand::Immediate(imm)) = instructions[14] {
        imm
    } else {
        panic!("Fifteenth instruction is not add y <imm>")
    };

    assert!(div == 26 || div == 1);
    if div == 26 {
        assert!(add1 <= 9);
        InstructionBlockType::Type2(add1, add2)
    } else {
        assert!(add1 > 9);
        InstructionBlockType::Type1(add2)
    }
}

fn execute_alu_program(
    program: impl IntoIterator<Item = impl Borrow<Instruction>>,
    input: impl IntoIterator<Item = impl Borrow<i64>>,
) -> Result<[i64; 4]> {
    let mut variables = [0i64; 4];

    let mut input = input.into_iter();

    for instruction in program.into_iter() {
        match *instruction.borrow() {
            Instruction::Inp(var) => {
                variables[var as usize] = *input.next().context("Missing input")?.borrow();
            }
            Instruction::Add(var, operand) => {
                variables[var as usize] += match operand {
                    Operand::Variable(var) => variables[var as usize],
                    Operand::Immediate(imm) => imm,
                };
            }
            Instruction::Mul(var, operand) => {
                variables[var as usize] *= match operand {
                    Operand::Variable(var) => variables[var as usize],
                    Operand::Immediate(imm) => imm,
                };
            }
            Instruction::Div(var, operand) => {
                let operand = match operand {
                    Operand::Variable(var) => variables[var as usize],
                    Operand::Immediate(imm) => imm,
                };
                if operand == 0 {
                    bail!("Division by 0");
                }

                variables[var as usize] /= operand;
            }
            Instruction::Mod(var, operand) => {
                let operand = match operand {
                    Operand::Variable(var) => variables[var as usize],
                    Operand::Immediate(imm) => imm,
                };
                if operand == 0 {
                    bail!("Division by 0");
                }
                if variables[var as usize] < 0 || operand < 0 {
                    bail!("Negative modulo operation");
                }

                variables[var as usize] %= operand;
            }
            Instruction::Eql(var, operand) => {
                let operand = match operand {
                    Operand::Variable(var) => variables[var as usize],
                    Operand::Immediate(imm) => imm,
                };

                variables[var as usize] = if variables[var as usize] == operand {
                    1
                } else {
                    0
                };
            }
        }
    }

    Ok(variables)
}

fn parse_input() -> Result<Vec<Instruction>> {
    input_lines()?
        .map(|line| -> Result<Instruction> { Ok(line?.parse()?) })
        .collect()
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(inp|add|mul|div|mod|eql) (x|y|z|w)(?: (x|y|z|w|-?\d+)|)$").unwrap();
        }

        let captures = RE.captures(s).context("Invalid instruction string")?;

        let variable = match captures.get(2).unwrap().as_str() {
            "x" => Variable::X,
            "y" => Variable::Y,
            "z" => Variable::Z,
            "w" => Variable::W,
            _ => unreachable!(),
        };

        let operand = captures.get(3).map(|operand| match operand.as_str() {
            "x" => Operand::Variable(Variable::X),
            "y" => Operand::Variable(Variable::Y),
            "z" => Operand::Variable(Variable::Z),
            "w" => Operand::Variable(Variable::W),
            number => Operand::Immediate(number.parse().unwrap()),
        });

        let opcode = match captures.get(1).unwrap().as_str() {
            "inp" => Instruction::Inp(variable),
            "add" => Instruction::Add(variable, operand.context("Missing operand")?),
            "mul" => Instruction::Mul(variable, operand.context("Missing operand")?),
            "div" => Instruction::Div(variable, operand.context("Missing operand")?),
            "mod" => Instruction::Mod(variable, operand.context("Missing operand")?),
            "eql" => Instruction::Eql(variable, operand.context("Missing operand")?),
            _ => unreachable!(),
        };

        Ok(opcode)
    }
}

#[cfg(test)]
mod tests {
    use super::{execute_alu_program, Instruction, Operand, Variable};

    #[test]
    fn alu_sanity() {
        assert_eq!(
            execute_alu_program(
                [
                    Instruction::Inp(Variable::X),
                    Instruction::Mul(Variable::X, Operand::Immediate(-1)),
                ],
                [42],
            )
            .unwrap(),
            [-42, 0, 0, 0]
        );

        assert_eq!(
            execute_alu_program(
                [
                    Instruction::Inp(Variable::Z),
                    Instruction::Inp(Variable::X),
                    Instruction::Mul(Variable::Z, Operand::Immediate(3)),
                    Instruction::Eql(Variable::Z, Operand::Variable(Variable::X)),
                ],
                [1, 3],
            )
            .unwrap(),
            [3, 0, 1, 0]
        );

        assert_eq!(
            execute_alu_program(
                [
                    Instruction::Inp(Variable::Z),
                    Instruction::Inp(Variable::X),
                    Instruction::Mul(Variable::Z, Operand::Immediate(3)),
                    Instruction::Eql(Variable::Z, Operand::Variable(Variable::X)),
                ],
                [1, 4],
            )
            .unwrap(),
            [4, 0, 0, 0]
        );
    }
}
