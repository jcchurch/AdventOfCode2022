use std::io;
use std::io::prelude::*;

struct Instruction {
    alias: String,
    cycles: i32,
    parameter: i32
}

fn read_instruction(line: String) -> Instruction {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts[0] == "addx" {
        return Instruction {
            alias: parts[0].to_string(),
            cycles: 2,
            parameter: parts[1].parse().unwrap()
        };
    }

    Instruction {
        alias: parts[0].to_string(),
        cycles: 1,
        parameter: 0
    }
}

fn read_all_instructions() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        instructions.push( read_instruction(line.unwrap()) );
    }

    instructions
}

fn compute_instructions(instructions: &Vec<Instruction>) -> i32 {
    let mut cycle_counter: i32 = 0;
    let mut x: i32 = 1;
    let mut power: i32 = 0;

    for instruction in instructions {
        let mut cycle_buffer: i32 = instruction.cycles;

        while cycle_buffer > 0 {
            cycle_counter += 1;
            cycle_buffer -= 1;

            if cycle_counter == 20 || cycle_counter == 60 || cycle_counter == 100 || cycle_counter == 140 || cycle_counter == 180 || cycle_counter == 220 {
                power += cycle_counter * x;
                println!("Cycle: {} X: {} Current: {} New Power: {}", cycle_counter, x, cycle_counter * x,  power);
            }
        }

        if instruction.alias == "addx" {
            x += instruction.parameter;
        }
    }

    power
}

fn main() {
    let instructions = read_all_instructions();
    let power = compute_instructions(&instructions);
    println!("{}", power);
}
