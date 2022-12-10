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

fn is_cycle_on_pixel(cycle_counter: i32, x: i32) -> bool {
    let horizontal = cycle_counter % 40;
    x == horizontal-1 || x == horizontal || x == horizontal+1
}

fn compute_instructions(instructions: &Vec<Instruction>) -> [char; 240] {
    let mut cycle_counter: i32 = 0;
    let mut x: i32 = 1;
    let mut video_buffer: [char; 240] = ['.'; 240];

    for instruction in instructions {
        let mut cycle_buffer: i32 = instruction.cycles;

        while cycle_buffer > 0 {
            cycle_counter += 1;
            cycle_buffer -= 1;

            if is_cycle_on_pixel(cycle_counter-1, x) {
                video_buffer[(cycle_counter-1) as usize] = '#';
            }
        }

        if instruction.alias == "addx" {
            x += instruction.parameter;
        }
    }

    video_buffer
}

fn main() {
    let instructions = read_all_instructions();
    let video_buffer = compute_instructions(&instructions);
    for i in 0..6 {
        let line: String = video_buffer[i*40..(i+1)*40].iter().collect();
        println!("{}", line);
    }
}
