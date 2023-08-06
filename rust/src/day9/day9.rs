#![allow(dead_code)]

use std::fmt::Debug;
use std::fmt::Display;
use std::result::Result;

use std::iter::Iterator;
use std::collections::VecDeque;

// use permutations
use itertools::Itertools;

// console output coloring
use colorful::Color;
use colorful::Colorful;

// allow use of io crate
use std::io;
use std::io::Write;


#[derive(Debug, PartialEq, Copy, Clone)]
enum Mode {
    Immediate(isize),
    Position(usize),
    Relative(isize),
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Mode::Immediate(value) => format!("{}({})", "Imm".color(Color::SlateBlue3a), value),
            Mode::Position(pos) => format!("{}({})", "Pos".color(Color::Green3a), pos),
            Mode::Relative(offset) => format!("{}({})", "Rel".color(Color::SlateBlue1), offset),
        };
        write!(f, "{}", s)
    }
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

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Opcode::Add => "Add",
            Opcode::Multiply => "Multiply",
            Opcode::Input => "Input",
            Opcode::Output => "Output",
            Opcode::Jump => "Jump",
            Opcode::JumpNot => "JumpNot",
            Opcode::LessThan => "LessThan",
            Opcode::Equals => "Equals",
            Opcode::Halt => "Halt",
        };
        let s = format!("{}", s.color(Color::SpringGreen3b));
        write!(f, "{}", s)
    }
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

#[derive(Debug, Clone)]
enum MachineState
{
    Booting,
    Running,
    Halted,
    Stalled, // waiting for input
    Corrupted{ reason: String },
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

#[derive(Debug, Clone)]
struct Memory {
    memory: Vec<isize>,
    offset: usize,
}

// create a trait for a Source. This is a source of input for the machine.
trait Source {
    fn read(&mut self) -> Option<isize>;
}

trait Sink {
    fn write(&mut self, value: isize);
}

type MachineId = usize;

#[derive(Debug)]
struct Machine {
    memory: Memory,
    id: MachineId,
    state: MachineState,
    relative_base: isize,
}

impl Machine {
    fn new(memory: Memory, id: usize) -> Self {
        Self {
            memory,
            id,
            state: MachineState::Booting,
            relative_base: 0,
        }
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn to_string(&self) -> String {
        format!("{} #{}", "Machine".color(Color::SkyBlue1), format!("{}", self.get_id()).color(Color::PaleGreen1a))
    }
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

    // Memory to string, with instruction. if instruction has memory reference, show value as color green.
    fn to_string(&self, instruction: &Instruction, machine: &Machine) -> String {
        let colors = vec![Color::Red, Color::DeepSkyBlue3a, Color::DeepSkyBlue3b, Color::DeepSkyBlue4a, Color::DeepSkyBlue4b, Color::DeepSkyBlue4c];
        let mut color = colors.iter().cycle();
        let mut offsets_to_color = vec![(self.offset, *color.next().unwrap())];
        
        // Results stored in a vector containing values implementing the StrMarker trait.
        let mut repr: Vec<Box<dyn Display>> = Vec::new();
        repr.push(Box::new("["));

        let mut configure_for_color = |mode, allow_imm| match mode {
            Mode::Position(pos) => {
                offsets_to_color.push((pos, *color.next().unwrap()));
            },
            Mode::Immediate(val) => {
                if allow_imm {
                    offsets_to_color.push((val as usize, *color.next().unwrap()));
                }
            },
            Mode::Relative(offset) => {
                offsets_to_color.push(((machine.relative_base + offset) as usize, *color.next().unwrap()));
            },
        };
            
        match instruction {
            Instruction::Trinary { code: _, lhs, rhs, dst } => {
                configure_for_color(*lhs, false);
                configure_for_color(*rhs, false);
                configure_for_color(*dst, false);
            },
            Instruction::Binary { code, lhs, rhs } => {
                match code {
                    Opcode::Jump | Opcode::JumpNot => {
                        configure_for_color(*lhs, true);
                        configure_for_color(*rhs, true);
                    },
                    _ => {
                        configure_for_color(*lhs, false);
                        configure_for_color(*rhs, false);
                    },
                }
            },
            Instruction::Unary { code: _, src } => {
                configure_for_color(*src, false);
            },
            _ => { },
        }

        for (offset, value) in self.memory.iter().enumerate() {
            if offset > 0 {
                repr.push(Box::new(", "));
            }
            
            if let Some((_, color)) = offsets_to_color.iter().find(|(pos, _)| *pos == offset) {
                repr.push(Box::new(format!("{}", value).color(*color)));
            } else {
                repr.push(Box::new(format!("{}", value)));
            }
        }
        
        repr.push(Box::new("]"));
        repr.into_iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join("")
    }
}

impl Instruction {
    // fn execute(&self, context: &mut Machine) -> Result<(), String> {
    fn execute(&self, context: &mut Machine, input: &mut dyn Source, output: &mut dyn Sink) -> Result<(), String> {
        let dereference = |value| -> isize {
            match value {
                Mode::Position(pos) => context.memory.get(pos as usize),
                Mode::Immediate(value) => value,
                Mode::Relative(offset) => context.memory.get((context.relative_base + offset) as usize),
            }
        };

        match self {
            Instruction::Trinary { code, lhs, rhs, dst } => {
                if let Mode::Immediate(_) = dst {
                    panic!("Invalid destination mode: {:?}", dst);
                }

                let (lhs, rhs) = (dereference(*lhs), dereference(*rhs));
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
                context.memory.set(write, result);
                Ok(())
            },

            Instruction::Binary { code, lhs, rhs } => {
                let (lhs, rhs) = (dereference(*lhs), dereference(*rhs));

                match code {
                    Opcode::Jump => {
                        if lhs != 0 {
                            context.memory.offset = rhs as usize;
                        }
                    },
                    Opcode::JumpNot => {
                        if lhs == 0 {
                            context.memory.offset = rhs as usize;
                        }
                    },
                    _ => unreachable!(),
                }

                Ok(())
            },

            Instruction::Unary { code, src } => {
                match code {
                    Opcode::Input => {
                        match input.read() {
                            Some(value) => {
                                println!("{}", format!("Input: {}", format!("{}", value).color(Color::Yellow)).color(Color::Yellow3a));
                                let store = match src {
                                    Mode::Position(pos) => *pos,
                                    _ => unreachable!(),
                                };
                                context.memory.set(store as usize, value);
                                Ok(())
                            },
                            None => {
                                context.state = MachineState::Stalled;
                                Ok(())
                            },
                        }
                    },
                    Opcode::Output => {
                        println!("{}", format!("Machine output: {}", dereference(*src)).color(Color::SkyBlue1));
                        output.write(dereference(*src));
                        Ok(())
                    },
                    _ => unreachable!(),
                }
            },

            Instruction::Halt => {
                context.state = MachineState::Halted;
                Ok(())
            },
        }
    }

    fn to_string_with_memory(&self, memory: &Memory, machine: &Machine) -> String {
        let mut result = String::new();

        let mode_to_string = |mode| match mode {
            Mode::Immediate(value) => format!("{}({})", "Imm".color(Color::PaleGreen1a), value),
            Mode::Position(pos) => format!("{}({})={} ", "Pos".color(Color::PaleGreen1a), pos, memory.get(pos)),
            Mode::Relative(offset) => format!("{}({})={} ", "Rel".color(Color::PaleGreen1a), offset, memory.get((machine.relative_base + offset) as usize)),
        };

        match self {
            Instruction::Trinary { code, lhs, rhs, dst } => {
                result.push_str(&format!("{} {} {} {}", code, mode_to_string(*lhs), mode_to_string(*rhs), mode_to_string(*dst)));
            },
            Instruction::Binary { code, lhs, rhs } => {
                result.push_str(&format!("{} {} {}", code, mode_to_string(*lhs), mode_to_string(*rhs)));
            },
            Instruction::Unary { code, src } => {
                result.push_str(&format!("{} {}", code, mode_to_string(*src)));
                if let Mode::Position(pos) = src {
                    if let Opcode::Input = code {
                        result.push_str(&format!(" (input loc: {})", memory.get(*pos)));
                    }
                }
            },
            Instruction::Halt => {
                result.push_str("Halt");
            },
        }

        result
    }
}

fn run(context: &mut Machine, input: &mut dyn Source, output: &mut dyn Sink)
{
    context.state = MachineState::Running;

    let to_mode = |mode, arg| match mode {
        0 => Mode::Position(arg as usize),
        1 => Mode::Immediate(arg as isize),
        2 => Mode::Relative(arg as isize),
        _ => panic!("Invalid mode: {}", mode),
    };

    loop {
        let instr = context.memory.get(context.memory.offset);
        let opcode = instr % 100;
        let (instruction, increment) = match opcode {
            1..=2 | 7..=8 => {
                let (lhs, rhs, dst) = (
                    to_mode(instr / 100 % 10, context.memory.get(context.memory.offset + 1)),
                    to_mode(instr / 1000 % 10, context.memory.get(context.memory.offset + 2)),
                    to_mode(instr / 10000 % 10, context.memory.get(context.memory.offset + 3)),
                );
                (Instruction::Trinary { code: opcode.into(), lhs, rhs, dst, }, 4)
            },
            3..=4 => {
                let src = to_mode(instr / 100 % 10, context.memory.get(context.memory.offset + 1));
                (Instruction::Unary { code: opcode.into(), src, }, 2)
            },
            5..=6 => {
                let (src, dst) = (
                    to_mode(instr / 100 % 10, context.memory.get(context.memory.offset + 1)),
                    to_mode(instr / 1000 % 10, context.memory.get(context.memory.offset + 2)),
                );
                (Instruction::Binary { code: opcode.into(), lhs: src, rhs: dst, }, 3)
            },
            99 => (Instruction::Halt, 1),
            _ => {
                context.state = MachineState::Corrupted{ reason: format!("Invalid opcode: {}", opcode) };
                return;
            }
        };

        let instruction_pointer = context.memory.offset;
        let previous_fmt = format!("(Pre)  Memory: {}", context.memory.to_string(&instruction, &context));
        let instruction_info = format!("{}; Instr Pointer: {}",
                                       instruction.to_string_with_memory(&context.memory, &context).color(Color::Green),
                                       format!("{}", instruction_pointer).color(Color::PaleGreen1a));
        let _ = instruction.execute(context, input, output);

        match context.state.clone() {
            MachineState::Running => {
                println!("{} => {}", instruction_info, format!("{}", context.memory.offset).color(Color::PaleGreen1a));
                println!("{}", previous_fmt);
                println!("(Post) Memory: {}", context.memory.to_string(&instruction, &context));
                println!();
                
                // If the instruction pointer was not modified, increment it by the instruction size.
                // Otherwise, the instruction pointer was modified by the instruction.
                if instruction_pointer == context.memory.offset {
                    context.memory.offset += increment;
                }

                if let Instruction::Halt = instruction {
                    context.memory.offset -= increment;
                    return;
                }
            },
            MachineState::Stalled => {
                println!("{} => {} -- Stalled", instruction_info, format!("{}", context.memory.offset).color(Color::PaleGreen1a));
                return;
            },
            MachineState::Corrupted { reason } => {
                println!("{} => {} -- Corruption", instruction_info, format!("{}", context.memory.offset).color(Color::PaleGreen1a));
                panic!("Machine corrupted: {}", reason.color(Color::Red));
            },
            MachineState::Halted => {
                println!("{} => {} -- Halted", instruction_info, format!("{}", context.memory.offset).color(Color::PaleGreen1a));
                println!("{}", "Machine Halted".color(Color::SkyBlue1));
                return;
            },
            _ => todo!(),
        }
    }
}

struct ConsoleSource {
    color: Color,
}

struct ConsoleSink {
    color: Color,
}

impl ConsoleSource {
    fn new(color: Color) -> Self {
        Self { color }
    }
}

impl ConsoleSink {
    fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Source for ConsoleSource {
    fn read(&mut self) -> Option<isize> {
        let mut input = String::new();
        print!("{}: ", "Input".color(self.color));
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().ok()
    }
}

impl Sink for ConsoleSink {
    fn write(&mut self, value: isize) {
        println!("{}: {}", "Output".color(self.color), value);
    }
}

struct MemoryBus {
    queue: VecDeque<isize>,
}

impl std::fmt::Display for MemoryBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str("MemoryBus{ [");
        result.push_str(&self.queue.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(", "));
        result.push_str("] }");
        write!(f, "{}", result)
    }
}

impl MemoryBus {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn seed(&mut self, value: isize) {
        self.queue.push_back(value);
    }
}

impl Source for MemoryBus {
    fn read(&mut self) -> Option<isize> {
        self.queue.pop_front()
    }
}

impl Sink for MemoryBus {
    fn write(&mut self, value: isize) {
        self.queue.push_back(value);
    }
}

fn main() {
    println!("Advent of Code 2019 - Day 9");

    if let Some(arg) = std::env::args().nth(1) {
        println!("Reading program from file: {}", arg);
        let program = std::fs::read_to_string(arg).expect("Failed to read file");
        let program: Vec<isize> = program.split(',').map(|s| s.trim().parse().expect("Failed to parse integer")).collect();

        if let Some(program_kind) = std::env::args().nth(2) {
            match program_kind.as_str() {
                "regular" => {
                    let memory = Memory::new(program.clone());
                    let mut context = Machine::new(memory, 0);
                    let mut input = ConsoleSource::new(Color::PaleGreen1a);
                    let mut output = ConsoleSink::new(Color::PaleGreen1a);
                    loop {
                        run(&mut context, &mut input, &mut output);

                        if let MachineState::Halted = context.state {
                            break;
                        }
                    }
                },
                "amplify" => {
                    let mut max = 0;
                    for permutation in (0..=4).permutations(5) {
                        let memory = Memory::new(program.clone());
                        let mut machines = Vec::new();
                        let mut buses = Vec::new();
                        let mut solution = MemoryBus::new();

                        for i in 0..5 {
                            buses.push(MemoryBus::new());
                            buses[i].seed(permutation[i]); // phase setting
                            if i == 0 {
                                buses[i].seed(0);
                            }
                            machines.push(Machine::new(memory.clone(), i));
                        }

                        let count_buses = buses.len();
                        if count_buses != 5 {
                            panic!("Expected 5 buses, got {}", count_buses);
                        }

                        // Create N machines. Have each machine's output feed into the next machine's input.
                        // The first machine's input is seeded with 0.
                        // The last machine's output is the final output.
                        let mut all_halted = false;
                        while !all_halted {
                            all_halted = true;
                            for (i, machine) in machines.iter_mut().enumerate() {
                                println!("{}: Running. {}", machine.to_string(), buses[i]);
                                if i + 1 >= count_buses {
                                    let input_bus = &mut buses[i];
                                    let output_bus = &mut solution;
                                    run(machine, input_bus, output_bus);
                                } else {
                                    let (left, right) = buses.split_at_mut(i + 1);
                                    let (left_offset, right_offset) = (left.len() - 1, left.len());
                                    let input_bus = &mut left[left_offset];
                                    let output_bus = &mut right[0];
                                    println!("{}: Bus offsets: {},{}", machine.to_string(), left_offset, right_offset);
                                    run(machine, input_bus, output_bus);
                                }

                                if let MachineState::Halted = machine.state {
                                    // do nothing
                                } else {
                                    all_halted &= false;
                                }

                                println!("{}: Stopped. {}", machine.to_string(), buses[i]);
                                println!("{}: All Bus states: {}", machine.to_string(), buses.iter().map(|b| format!("{}", b)).collect::<Vec<_>>().join(", "));
                            }
                        }

                        let output = solution.read().unwrap();
                        max = std::cmp::max(max, output);

                        for (_i, bus) in buses.iter().enumerate() {
                            println!("{} : MemoryBus {}:", format!("Permutation {}", permutation.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(", ")).color(Color::PaleGreen1a), bus);
                        }
                    }
                    println!("Max: {}", max);
                },
                
                "feedback" => {
                    let mut max = 0;
                    for permutation in (5..=9).permutations(5) {
                        let memory = Memory::new(program.clone());
                        let mut machines = Vec::new();
                        let mut buses = Vec::new();

                        for i in 0..5 {
                            buses.push(MemoryBus::new());
                            buses[i].seed(permutation[i]); // phase setting
                            if i == 0 {
                                buses[i].seed(0);
                            }
                            machines.push(Machine::new(memory.clone(), i));
                        }

                        let count_buses = buses.len();
                        if count_buses != 5 {
                            panic!("Expected 5 buses, got {}", count_buses);
                        }

                        // Create N machines. Have each machine's output feed into the next machine's input.
                        // The first machine's input is seeded with 0.
                        // The last machine's output is the final output.
                        let mut all_halted = false;
                        while !all_halted {
                            all_halted = true;
                            for (i, machine) in machines.iter_mut().enumerate() {
                                println!("{}: Running. {}", machine.to_string(), buses[i]);
                                if i + 1 == count_buses {
                                    let (left, right) = buses.split_at_mut(1);
                                    let input_bus_offset = right.len() - 1 + left.len();
                                    let output_bus = &mut left[0];
                                    let input_bus = &mut right[right.len() - 1];
                                    println!("{}: Bus offsets: {},{}", machine.to_string(), input_bus_offset, 0);
                                    run(machine, input_bus, output_bus);
                                } else {
                                    let (left, right) = buses.split_at_mut(i + 1);
                                    let (left_offset, right_offset) = (left.len() - 1, left.len());
                                    let input_bus = &mut left[left_offset];
                                    let output_bus = &mut right[0];
                                    println!("{}: Bus offsets: {},{}", machine.to_string(), left_offset, right_offset);
                                    run(machine, input_bus, output_bus);
                                }

                                if let MachineState::Halted = machine.state {
                                    // do nothing
                                } else {
                                    all_halted &= false;
                                }

                                println!("{}: Stopped. {}", machine.to_string(), buses[i]);
                                println!("{}: All Bus states: {}", machine.to_string(), buses.iter().map(|b| format!("{}", b)).collect::<Vec<_>>().join(", "));
                            }
                        }

                        let output = buses[0].read().unwrap();
                        max = std::cmp::max(max, output);
                    }
                    println!("Max: {}", max);
                },

                _ => panic!("Invalid program kind: {}. Valid program kinds: regular, amplify", program_kind),
            }
        } else {
            println!("Usage: {} <program> <program kind>. Accepted program kinds: regular, amplify", std::env::args().nth(0).unwrap());
        }
    } else {
        println!("Running against test program.");
        let memory = Memory::new(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,
                                      1006,20,31,1106,0,36,98,0,0,1002,21,125,
                                      20,4,20,1105,1,46,104,999,1105,1,46,1101,
                                      1000,1,20,4,20,1105,1,46,98,99]);
        println!("Memory: {:?}", memory.data());

        let mut execution_context = Machine::new(memory, 0);
        let mut input = ConsoleSource::new(Color::Green);
        let mut output = ConsoleSink::new(Color::Green);
        run(&mut execution_context, &mut input, &mut output);
        println!("Memory: {:?}", execution_context.memory.data());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn golden() {
        let memory = Memory::new(vec![1,9,10,3,2,3,11,0,99,30,40,50]);
        let mut execution_context = Machine::new(memory, 0 as MachineId);
        let mut input = ConsoleSource::new(Color::Green);
        let mut output = ConsoleSink::new(Color::Green);
        println!("Memory: {:?}", execution_context.memory.data());
        run(&mut execution_context, &mut input, &mut output);
        println!("Memory: {:?}", execution_context.memory.data());
        assert_eq!(execution_context.memory.data(), vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
    }
}
