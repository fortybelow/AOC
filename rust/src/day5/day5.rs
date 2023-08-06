#![allow(dead_code)]

use std::fmt::Debug;
use std::result::Result;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Mode {
    Immediate(isize),
    Position(usize),
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    Jump,
    JumpNot,
    LessThan,
    Equals,
    Halt,
}

impl Into<Opcode> for isize {
    fn into(self) -> Opcode {
        match self {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::Jump,
            6 => Opcode::JumpNot,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Halt,
            _ => panic!("Invalid opcode: {}", self),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Trinary {
        code: Opcode,
        lhs: Mode,
        rhs: Mode,
        dst: Mode,
    },
    Binary {
        code: Opcode,
        lhs: Mode,
        rhs: Mode,
    },
    Unary {
        code: Opcode,
        src: Mode,
    },
    Halt,
}

struct Memory {
    memory: Vec<isize>,
    offset: usize,
}

impl Memory {
    fn new(memory: Vec<isize>) -> Self {
        Self {
            memory,
            offset: 0,
        }
    }

    fn get(&self, offset: usize) -> isize {
        *self.memory.get(offset).unwrap_or(&0)
    }

    fn set(&mut self, offset: usize, value: isize) {
        if offset >= self.memory.len() {
            self.memory.resize(offset + 1, 0);
        }
        self.memory[offset] = value;
    }

    fn data(&self) -> &[isize] {
        &self.memory
    }
}

impl Instruction {
    fn execute(&self, memory: &mut Memory) -> Result<(), String> {
        let to_value = |value| -> isize {
            match value {
                Mode::Position(pos) => memory.get(pos as usize),
                Mode::Immediate(value) => value,
            }
        };

        match self {
            Instruction::Trinary { code, lhs, rhs, dst } => {
                if let Mode::Immediate(_) = dst {
                    panic!("Invalid destination mode: {:?}", dst);
                }

                let (lhs, rhs) = (to_value(*lhs), to_value(*rhs));
                let result = match code {
                    Opcode::Add => lhs + rhs,
                    Opcode::Multiply => lhs * rhs,
                    Opcode::LessThan => (lhs < rhs) as isize,
                    Opcode::Equals => (lhs == rhs) as isize,
                    _ => unreachable!(),
                };
                let write = match dst {
                    Mode::Position(pos) => *pos,
                    _ => unreachable!(),
                };
                memory.set(write, result);
                Ok(())
            },

            Instruction::Binary { code, lhs, rhs } => {
                let (lhs, rhs) = (to_value(*lhs), to_value(*rhs));

                match code {
                    Opcode::Jump => {
                        if lhs != 0 {
                            memory.offset = rhs as usize;
                        }
                    },
                    Opcode::JumpNot => {
                        if lhs == 0 {
                            memory.offset = rhs as usize;
                        }
                    },
                    _ => unreachable!(),
                }

                Ok(())
            },

            Instruction::Unary { code, src } => {
                match code {
                    Opcode::Input => {
                        if let Mode::Immediate(_) = src {
                            panic!("Invalid source mode: {:?}", src);
                        }

                        let pos = match src {
                            Mode::Position(pos) => pos,
                            _ => unreachable!(),
                        };

                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect("Failed to read line");
                        let input = input.trim().parse().expect("Failed to parse integer");
                        memory.set(*pos as usize, input);
                        Ok(())
                    },
                    Opcode::Output => {
                        println!("{}", to_value(*src));
                        Ok(())
                    },
                    _ => unreachable!(),
                }
            },

            Instruction::Halt => Ok(()),
        }
    }
}

fn deserialize(memory: &mut Memory) -> Result<Vec<Instruction>, String>
{
    // iter of memory
    // let mut offset = 0usize;
    let mut instructions = Vec::new();

    let to_mode = |mode, arg| match mode {
        0 => Mode::Position(arg as usize),
        1 => Mode::Immediate(arg as isize),
        _ => panic!("Invalid mode: {}", mode),
    };

    loop {
        let instr = memory.get(memory.offset);
        let opcode = instr % 100;
        let (instruction, increment) = match opcode {
            1..=2 | 7..=8 => {
                let (lhs, rhs, dst) = (
                    to_mode(instr / 100 % 10, memory.get(memory.offset + 1)),
                    to_mode(instr / 1000 % 10, memory.get(memory.offset + 2)),
                    to_mode(instr / 10000 % 10, memory.get(memory.offset + 3)),
                );
                (Instruction::Trinary { code: opcode.into(), lhs, rhs, dst, }, 4)
            },
            3..=4 => {
                let src = to_mode(instr / 100 % 10, memory.get(memory.offset + 1));
                (Instruction::Unary { code: opcode.into(), src, }, 2)
            },
            5..=6 => {
                let (src, dst) = (
                    to_mode(instr / 100 % 10, memory.get(memory.offset + 1)),
                    to_mode(instr / 1000 % 10, memory.get(memory.offset + 2)),
                );
                (Instruction::Binary { code: opcode.into(), lhs: src, rhs: dst, }, 3)
            },
            99 => (Instruction::Halt, 1),
            _ => return Err(format!("Invalid opcode: {}", opcode)),
        };

        memory.offset += increment;
        let _previous_offset = memory.offset.clone();
        
        // println!("Instruction: {:?}", instruction);
        instruction.execute(memory)?;
        instructions.push(instruction);
        // println!("Executed instruction: {:?}; Previous Ip: {:?}; Ip: {:?}", instructions.last().unwrap(), previous_offset, memory.offset);
        // println!("Memory: {:?}", memory.data());
        // println!();

        if let Instruction::Halt = instructions.last().unwrap() {
            break;
        }
    }

    Ok(instructions)
}

fn main() {
    println!("Advent of Code 2019 - Day 5");

    // if has argument
    if let Some(arg) = std::env::args().nth(1) {
        let program = std::fs::read_to_string(arg).expect("Failed to read file");
        let program: Vec<isize> = program.split(',').map(|s| s.trim().parse().expect("Failed to parse integer")).collect();

        let mut memory = Memory::new(program);
        let _ = deserialize(&mut memory);

        println!("Memory: {:?}", memory.data());
    } else {
        // 3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        // 1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        // 999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
        let mut memory = Memory::new(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,
                                          1006,20,31,1106,0,36,98,0,0,1002,21,125,
                                          20,4,20,1105,1,46,104,999,1105,1,46,1101,
                                          1000,1,20,4,20,1105,1,46,98,99]);
        println!("Memory: {:?}", memory.data());

        let instructions = deserialize(&mut memory);
        println!("Memory: {:?}", memory.data());
        println!("Instructions: {:?}", instructions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let mut memory = Memory::new(vec![1,9,10,3,2,3,11,0,99,30,40,50]);
        println!("Memory: {:?}", memory.data());
        let instructions = deserialize(&mut memory).unwrap();
        println!("Instructions: {:?}", instructions);
        println!("Memory: {:?}", memory.data());
        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[0], Instruction::Trinary { code: Opcode::Add, lhs: Mode::Position(9usize), rhs: Mode::Position(10usize), dst: Mode::Position(3usize), });
        assert_eq!(instructions[1], Instruction::Trinary { code: Opcode::Multiply, lhs: Mode::Position(3usize), rhs: Mode::Position(11usize), dst: Mode::Position(0usize), });
        assert_eq!(instructions[2], Instruction::Halt);
        assert_eq!(memory.data(), vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
    }
}
