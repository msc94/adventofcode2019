use std::{error::Error, iter::Zip};
use itertools::Itertools;

enum Opcode {
    ADD = 1,
    MULTIPLY = 2,
    STOP = 99,
}

impl TryFrom<usize> for Opcode {
    type Error = String;

    fn try_from(v: usize) -> Result<Opcode, String> {
        match v {
            x if x == Opcode::ADD as usize => Ok(Opcode::ADD),
            x if x == Opcode::MULTIPLY as usize => Ok(Opcode::MULTIPLY),
            x if x == Opcode::STOP as usize => Ok(Opcode::STOP),
            _ => Err("Unknown opcode".to_string()),
        }
    }
}

fn read_memory(memory: &Vec<usize>, addr: usize) -> Result<usize, Box<dyn Error>> {
    Ok(memory
        .get(addr)
        .ok_or(format!("Can't read from address {addr}"))?
        .clone())
}

fn write_memory(memory: &mut Vec<usize>, addr: usize, value: usize) -> Result<(), Box<dyn Error>> {
    let elem = memory
        .get_mut(addr)
        .ok_or(format!("Can't write to address {addr}"))?;
    *elem = value;
    Ok(())
}

fn execute(memory: &mut Vec<usize>) -> Result<usize, Box<dyn Error>> {
    let mut pc = 0;

    loop {
        // println!("Current state (PC = {pc})");
        // dbg!(&memory);

        let current_opcode = memory[pc].try_into()?;
        match current_opcode {
            Opcode::ADD => {
                let a = read_memory(memory, memory[pc + 1])?;
                let b = read_memory(memory, memory[pc + 2])?;
                write_memory(memory, memory[pc + 3], a + b)?;
                pc += 4;
            }
            Opcode::MULTIPLY => {
                let a = read_memory(memory, memory[pc + 1])?;
                let b = read_memory(memory, memory[pc + 2])?;
                write_memory(memory, memory[pc + 3], a * b)?;
                pc += 4;
            }
            Opcode::STOP => {
                return Ok(read_memory(memory, 0)?);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let initial_memory: Vec<usize> = utils::get_input("day2.txt")?
        .split(",")
        .map(|x| x.trim().parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;

    for (noun, verb) in (0..100).cartesian_product(0..100) {
        let mut memory = initial_memory.clone();
        write_memory(&mut memory, 1, noun)?;
        write_memory(&mut memory, 2, verb)?;
        if execute(&mut memory)? == 19690720 {
            println!("Correct: {}", 100 * noun + verb);
            break;
        } else {
            println!("Wrong: {}, {}", noun, verb);
        }
    }

    Ok(())
}
