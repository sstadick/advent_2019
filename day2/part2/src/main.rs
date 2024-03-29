use snafu::{ResultExt, Snafu};
use std::io::prelude::*;
use std::io::{self};

#[derive(Debug, Snafu)]
enum IntCodeError {
    #[snafu(display("Attempted to read memory out of bounds: {}", index))]
    MemReadOutOfBounds { index: usize },
}

type Result<T, E = IntCodeError> = std::result::Result<T, E>;

enum Instruction {
    Add(Parameters),
    Mult(Parameters),
    Halt,
}

struct Parameters {
    lhs: usize,
    rhs: usize,
    result: usize,
}

fn read_memory(memory: &[usize]) -> Vec<Instruction> {
    let mut index = 0;
    let mut ops = vec![];
    loop {
        if index >= memory.len() {
            //return Err(IntCodeError::MemReadOutOfBounds { index });
            break;
        }
        let code = memory[index];
        let op = match code {
            1 => {
                let op = Instruction::Add(Parameters {
                    lhs: memory[index + 1],
                    rhs: memory[index + 2],
                    result: memory[index + 3],
                });
                index += 4;
                op
            }
            2 => {
                let op = Instruction::Mult(Parameters {
                    lhs: memory[index + 1],
                    rhs: memory[index + 2],
                    result: memory[index + 3],
                });
                index += 4;
                op
            }
            99 => {
                index += 1;
                Instruction::Halt
            }
            _ => {
                panic!("Illegal operation {}", code);
            }
        };
        ops.push(op);
    }
    ops
}

fn run_program(instructions: Vec<Instruction>, symbols: &mut Vec<usize>) -> Result<()> {
    for int in instructions {
        match int {
            Instruction::Add(params) => {
                match symbols.len() {
                    x if params.result >= x => {
                        return Err(IntCodeError::MemReadOutOfBounds {
                            index: params.result,
                        });
                    }
                    x if params.lhs >= x => {
                        return Err(IntCodeError::MemReadOutOfBounds { index: params.lhs });
                    }
                    x if params.rhs >= x => {
                        return Err(IntCodeError::MemReadOutOfBounds { index: params.rhs });
                    }
                    _ => {}
                }
                symbols[params.result] = symbols[params.lhs] + symbols[params.rhs];
            }
            Instruction::Mult(params) => {
                match symbols.len() {
                    x if params.result >= x => {
                        return Err(IntCodeError::MemReadOutOfBounds {
                            index: params.result,
                        });
                    }
                    x if params.lhs >= x => {
                        return Err(IntCodeError::MemReadOutOfBounds { index: params.lhs });
                    }
                    x if params.rhs >= x => {
                        return Err(IntCodeError::MemReadOutOfBounds { index: params.rhs });
                    }
                    _ => {}
                }
                symbols[params.result] = symbols[params.lhs] * symbols[params.rhs];
            }
            Instruction::Halt => break,
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    let memory = buffer
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    'outer: for x in 0..19690720 {
        'inner: for y in 0..19690720 {
            let mut memory = memory.clone();
            memory[1] = x;
            memory[2] = y;
            let instructions = read_memory(&memory);
            match run_program(instructions, &mut memory) {
                Ok(_) => {}
                Err(_) => continue, // skip programs that error out
            }
            match memory[0] {
                19690720 => {
                    println!("Answer: {}", 100 * x + y);
                    break 'outer;
                }
                x if x > 19690720 => break 'inner,
                _ => {}
            }
        }
    }
    Ok(())
}
