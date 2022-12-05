use std::io;
use std::io::prelude::*;

struct Cargo {
    piles: Vec< Vec<char> >
}

fn read_all_lines() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines.push( line.unwrap() );
    }

    lines
}

fn determine_number_of_bins(line: &String) -> u32 {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens[0] == "1" {
        return tokens[tokens.len()-1].parse().unwrap();
    }
    0
}

fn get_line_number_containing_bin_count(lines: &Vec<String>) -> usize {
    let mut i: usize = 0;
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens[0] == "1" {
            return i;
        }
        i += 1;
    }
    0
}

fn build_empty_cargo_piles(pile_count: u32) -> Cargo {
    let mut piles: Vec< Vec<char> > = Vec::new();
    for _ in 0..pile_count {
        let pile: Vec<char> = Vec::new();
        piles.push(pile);
    }
    Cargo { piles: piles }
}

fn build_cargo(lines: &Vec<String>) -> Cargo {
    let line_number: usize = get_line_number_containing_bin_count(&lines);
    let pile_count: u32 = determine_number_of_bins(&lines[line_number]);
    let mut cargo = build_empty_cargo_piles(pile_count);
    load_initial_cargo(&mut cargo, &lines, line_number);
    make_moves(&mut cargo, &lines, line_number);
    cargo
}

fn load_initial_cargo(cargo: &mut Cargo, lines: &Vec<String>, line_number: usize) {
    for i in (0..line_number).rev() {
        let chars: Vec<char> = lines[i].chars().collect();
        let mut mypos: usize = 1;
        let mut mybox: usize = 0;
        while mypos < chars.len() {
            if chars[mypos] != ' ' {
                cargo.piles[mybox].push(chars[mypos]);
            }
            mypos += 4;
            mybox += 1;
        }
    }
}

fn make_moves(cargo: &mut Cargo, lines: &Vec<String>, line_number: usize) {
    let mut moves_line: usize = line_number + 2;
    while moves_line < lines.len() {
        make_one_move(cargo, &lines[moves_line]);
        moves_line += 1;
    }
}

fn make_one_move(cargo: &mut Cargo, line: &String) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let count: u32 = tokens[1].parse().unwrap();
    let source: usize = tokens[3].parse().unwrap();
    let destination: usize = tokens[5].parse().unwrap();

    for _ in 0..count {
        let item: char = cargo.piles[source - 1].pop().unwrap();
        cargo.piles[destination - 1].push(item);
    }
}

fn get_top_letter(cargo: &Cargo) -> String {
    let mut result: String = String::new();
    for pile in &cargo.piles {
        result.push(pile[ pile.len() - 1 ]);
    }
    result
}

fn main() {
    let lines = read_all_lines();
    let cargo = build_cargo(&lines);
    println!("{}", get_top_letter(&cargo));
}
