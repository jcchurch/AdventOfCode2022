use std::io;
use std::io::prelude::*;
pub mod matrix;

fn read_all_lines() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines.push( line.unwrap() );
    }

    lines
}

fn main() {
    let lines = read_all_lines();
    let mut matrix = matrix::load_char_matrix(&lines);
    matrix.display();
    println!("H: {}, W: {}", matrix.height(), matrix.width());

    for x in 0..matrix.width() {
        matrix.set(x, 0, 'X');
    }

    println!("---------------------------");
    matrix.display();
}
