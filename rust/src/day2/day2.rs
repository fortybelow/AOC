#![allow(dead_code)]


macro_rules! show_me {
    ($e:expr) => {
        {
            let val = $e;
            println!("{:?}", val);
            val
        }
    };
}

use std::fmt::Debug;
use std::result::Result;

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Multiply,
    Halt,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Binary {
        code: Opcode,
        lhs: usize,
        rhs: usize,
        dst: usize,
    },
    Halt,
}

impl Instruction {
    fn execute(&self, memory: &mut Vec<usize>) -> Result<(), String> {
        match self {
            Instruction::Binary { code, lhs, rhs, dst } => {
                let lhs = memory[*lhs];
                let rhs = memory[*rhs];
                let result = match code {
                    Opcode::Add => lhs + rhs,
                    Opcode::Multiply => lhs * rhs,
                    _ => unreachable!(),
                };
                memory[*dst] = result;
                Ok(())
            },
            Instruction::Halt => Ok(()),
        }
    }
}

// deserialize from Iterator of integers
fn deserialize(memory: &mut Vec<usize>) -> Result<Vec<Instruction>, String>
{
    // iter of memory
    let mut offset = 0usize;
    let mut instructions = Vec::new();

    loop {
        let opcode = *memory.get(offset).ok_or("Missing opcode")?;
        offset += 1;

        let instruction = match opcode {
            1..=2 => {
                let lhs = *memory.get(offset + 0).ok_or(format!("{} Missing lhs", opcode))?;
                let rhs = *memory.get(offset + 1).ok_or(format!("{} Missing rhs", opcode))?;
                let dst = *memory.get(offset + 2).ok_or(format!("{} Missing dst", opcode))?;
                offset += 3;

                let code = match opcode {
                    1 => Opcode::Add,
                    2 => Opcode::Multiply,
                    _ => unreachable!(),
                };
                Instruction::Binary { code, lhs, rhs, dst, }
            },
            99 => Instruction::Halt,
            _ => return Err(format!("Invalid opcode: {}", opcode)),
        };
        
        instruction.execute(memory)?;
        instructions.push(instruction);

        if let Instruction::Halt = instructions.last().unwrap() {
            break;
        }
    }

    Ok(instructions)
}

fn main() {
    // read program from first argument
    let program = std::env::args().nth(1).expect("Missing program argument");
    let program = std::fs::read_to_string(program).expect("Failed to read file");
    let program: Vec<usize> = program.split(',').map(|s| s.trim().parse().expect("Failed to parse integer")).collect();

    // part 1
    let mut memory = program.clone();
    memory[1] = 12;
    memory[2] = 2;
    deserialize(&mut memory);
        
    // print Vec<usize> as comma-separated list
    println!("{}", memory.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));

    // part 2
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = program.clone();
            memory[1] = noun;
            memory[2] = verb;
            deserialize(&mut memory);
            if memory[0] == 19690720 {
                println!("noun: {}, verb: {}", noun, verb);
                println!("answer: {}", 100 * noun + verb);
                return;
            }
        }
    }
}
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let mut memory = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let instructions = deserialize(&mut memory).unwrap();
        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[0], Instruction::Binary { code: Opcode::Add, lhs: 9, rhs: 10, dst: 3, });
        assert_eq!(instructions[1], Instruction::Binary { code: Opcode::Multiply, lhs: 3, rhs: 11, dst: 0, });
        assert_eq!(instructions[2], Instruction::Halt);
        assert_eq!(memory, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
    }
}
