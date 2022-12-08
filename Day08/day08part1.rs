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

fn char_diff(a: char, b: char) -> i32 {
    b as i32 - a as i32
}

fn is_taller(trees: &matrix::Matrix<char>, x: usize, y: usize, a: usize, b: usize) -> bool {
    char_diff( trees.get(a,b),  trees.get(x, y) ) > 0
}

fn populate_visible(trees: &matrix::Matrix<char>, visible: &mut matrix::Matrix<i32>) {
    for x in 0..trees.width() {
        visible.set(x, 0, 1);
        visible.set(x, trees.height()-1, 1);
    }

    for y in 0..trees.height() {
        visible.set(0, y, 1);
        visible.set(trees.width()-1, y, 1);
    }

    // North, moving South
    for x in 1..(trees.width()-1) {
        let mut tallest: usize = 0;
        for y in 1..trees.height() {
            if is_taller(&trees, x, y, x, tallest) {
                visible.set(x, y, 1);
                tallest = y;
            }
        }
    }

    // South, moving North
    for x in 1..(trees.width()-1) {
        let mut tallest: usize = trees.width()-1;
        for y in (0..(trees.height()-1)).rev() {
            if is_taller(&trees, x, y, x, tallest) {
                visible.set(x, y, 1);
                tallest = y;
            }
        }
    }
 
    // West, moving East 
    for y in 1..(trees.height()-1) {
        let mut tallest: usize = 0;
        for x in 1..trees.width() {
            if is_taller(&trees, x, y, tallest, y) {
                visible.set(x, y, 1);
                tallest = x;
            }
        }
    }

    // East, moving West
    for y in 1..(trees.height()-1) {
        let mut tallest: usize = trees.width()-1;
        for x in (0..trees.width()-1).rev() {
            if is_taller(&trees, x, y, tallest, y) {
                visible.set(x, y, 1);
                tallest = x;
            }
        }
    }
}

fn main() {
    let lines = read_all_lines();
    let trees: matrix::Matrix<char> = matrix::load_char_matrix(&lines);
    let mut visible: matrix::Matrix<i32> = matrix::build(trees.width(), trees.height(), 0);

    populate_visible(&trees, &mut visible);

    visible.display();
    println!("---------------------------");
    let mut total: i32 = 0;
    for y in 0..trees.height() {
        for x in 0..trees.width() {
            total += visible.get(x, y);
        }
    }

    println!("{}", total);
}
