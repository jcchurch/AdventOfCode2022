use std::io;
use std::io::prelude::*;
pub mod matrix;

struct Position {
    x: usize,
    y: usize,
    elevation: i32,
    steps: i32
}

fn read_all_lines() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines.push( line.unwrap() );
    }

    lines
}

fn char_to_elevation(ch: char) -> i32 {
    ch as i32 - 'a' as i32 + 1
}

fn find_peak(elevation_matrix: &matrix::Matrix<i32>) -> Position {
    for y in 0..elevation_matrix.height() {
        for x in 0..elevation_matrix.width() {
            if elevation_matrix.get(x, y) == 27 {
                return Position { x: x, y: y, elevation: 27, steps: 0 };
            }
        }
    }
    
    panic!("Cannot find peak.");
}

fn find_start(elevation_matrix: &matrix::Matrix<i32>) -> Position {
    for y in 0..elevation_matrix.height() {
        for x in 0..elevation_matrix.width() {
            if elevation_matrix.get(x, y) == 0 {
                return Position { x: x, y: y, elevation: 0, steps: 0 };
            }
        }
    }
    
    panic!("Cannot find peak.");
}

fn build_elevation_matrix(original_matrix: &matrix::Matrix<char>) -> matrix::Matrix<i32> {
    let mut elevation_matrix: matrix::Matrix<i32> = matrix::build( original_matrix.width(), original_matrix.height(), 0 );

    for y in 0..elevation_matrix.height() {
        for x in 0..elevation_matrix.width() {
            if original_matrix.get(x, y) == 'S' {
                elevation_matrix.set(x, y, 0);
            }
            else if original_matrix.get(x, y) == 'E' {
                elevation_matrix.set(x, y, 27);
            }
            else {
                elevation_matrix.set(x, y, char_to_elevation(original_matrix.get(x, y)));
            }
        }
    }

    elevation_matrix
}

fn potentially_add_position_to_queue(priority_queue: &mut Vec<Position>, elevation_matrix: &matrix::Matrix<i32>, x: usize, y: usize, elevation: i32, steps: i32) {
    if x < elevation_matrix.width() && y < elevation_matrix.height() && elevation_matrix.get(x, y) + 1 >= elevation {
        priority_queue.push( Position{  x: x, y: y, elevation: elevation_matrix.get(x, y), steps: steps + 1 } );
    }
}

fn build_path_matrix(elevation_matrix: &matrix::Matrix<i32>) -> matrix::Matrix<i32> {
    let mut path_matrix: matrix::Matrix<i32> = matrix::build( elevation_matrix.width(), elevation_matrix.height(), 0 );
    let the_end = find_peak(elevation_matrix);
    let mut priority_queue: Vec<Position> = Vec::new();
    priority_queue.push(the_end);

    while priority_queue.len() > 0 {
        let top = priority_queue.pop().unwrap();

        if path_matrix.get(top.x, top.y) == 0 {
            path_matrix.set(top.x, top.y, top.steps);

            if top.x >= 1 {
                potentially_add_position_to_queue(&mut priority_queue, elevation_matrix, top.x - 1, top.y    , top.elevation, top.steps);
            }

            if top.y >= 1 {
                potentially_add_position_to_queue(&mut priority_queue, elevation_matrix, top.x    , top.y - 1, top.elevation, top.steps);
            }

            potentially_add_position_to_queue(&mut priority_queue, elevation_matrix, top.x + 1, top.y    , top.elevation, top.steps);
            potentially_add_position_to_queue(&mut priority_queue, elevation_matrix, top.x    , top.y + 1, top.elevation, top.steps);

            priority_queue.sort_by(|a, b| b.steps.cmp(&a.steps));
        }
    }

    path_matrix
}

fn get_steps_to_end(elevation_matrix: &matrix::Matrix<i32>, path_matrix: &matrix::Matrix<i32>) -> i32 {
    let the_start = find_start(elevation_matrix);
    path_matrix.get(the_start.x, the_start.y)
}

fn main() {
    let lines = read_all_lines();
    let original_matrix = matrix::load_char_matrix(&lines);
    let elevation_matrix = build_elevation_matrix(&original_matrix);
    let path_matrix = build_path_matrix(&elevation_matrix);
    println!("{}", get_steps_to_end(&elevation_matrix, &path_matrix));
}
