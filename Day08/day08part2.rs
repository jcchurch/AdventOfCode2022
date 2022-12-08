use std::io;
use std::io::prelude::*;
use std::convert::TryInto;
pub mod matrix;

fn read_all_lines() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines.push( line.unwrap() );
    }

    lines
}

fn diff(a: char, b: char) -> i32 {
    b as i32 - a as i32
}

fn is_taller(trees: &matrix::Matrix<char>, x: usize, y: usize, a: usize, b: usize) -> bool {
    diff( trees.get(a,b),  trees.get(x, y) ) > 0
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

fn compute_tree_visibility(trees: &matrix::Matrix<char>, x: usize, y: usize) -> u32 {
    let mut north: u32 = 0;
    let mut south: u32 = 0;
    let mut east : u32 = 0;
    let mut west : u32 = 0;
    let mut xcursor: i32;
    let mut ycursor: i32;

    // Moving North
    ycursor = y as i32 - 1;
    while ycursor >= 0 && is_taller(&trees, x, y, x, ycursor as usize) {
        ycursor -= 1;
        north += 1;
    }
    if ycursor - 1 >= 0 {
        north += 1;
    }

    // Moving South
    ycursor = y as i32 + 1;
    while ycursor < trees.height().try_into().unwrap() && is_taller(&trees, x, y, x, ycursor as usize) {
        ycursor += 1;
        south += 1;
    }
    if ycursor < trees.height().try_into().unwrap() {
        south += 1;
    }

    // Moving East 
    xcursor = x as i32 + 1;
    while xcursor < trees.width().try_into().unwrap() && is_taller(&trees, x, y, xcursor as usize, y) {
        xcursor += 1;
        east += 1;
    }
    if xcursor < trees.width().try_into().unwrap() {
        east += 1;
    }

    // Moving West
    xcursor = x as i32 - 1;
    while xcursor >= 0 && is_taller(&trees, x, y, xcursor as usize, y) {
        xcursor -= 1;
        west += 1;
    }
    if xcursor - 1 >= 0 {
        west += 1;
    }

    north * south * east * west
}

fn main() {
    let lines = read_all_lines();
    let trees = matrix::load_char_matrix(&lines);
    let mut visible: matrix::Matrix<i32> = matrix::build(trees.width(), trees.height(), 0);

    populate_visible(&trees, &mut visible);

    visible.display();
    println!("---------------------------");
    let mut best_score = 0;
    for y in 0..trees.height() {
        for x in 0..trees.width() {
            if visible.get(x, y) > 0 {
                let score = compute_tree_visibility(&trees, x, y);
                if score > best_score {
                    best_score = score;
                }
            }
        }
    }
    println!("{}", best_score);
}
