use std::io::prelude::*;
use std::io::{self};

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

fn run_program(instructions: Vec<Instruction>, symbols: &mut Vec<usize>) {
    for int in instructions {
        match int {
            Instruction::Add(params) => {
                symbols[params.result] = symbols[params.lhs] + symbols[params.rhs];
            }
            Instruction::Mult(params) => {
                symbols[params.result] = symbols[params.lhs] * symbols[params.rhs];
            }
            Instruction::Halt => break,
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    let mut memory = buffer
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let instructions = read_memory(&memory);
    run_program(instructions, &mut memory);
    println!("Value at pos 0: {}", memory[0]);
    Ok(())
}
