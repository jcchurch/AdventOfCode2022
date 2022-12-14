use std::io;
use std::io::prelude::*;
pub mod matrix;

struct Coordinate {
    right: usize, 
    down: usize 
}

#[derive(Copy, PartialEq)]
enum Tank {
    Air, Wall, Sand, Source
}

impl Clone for Tank {
    fn clone(&self) -> Tank {
        *self
    }
}

struct Path {
    path: Vec<Coordinate>
}

fn read_path(line: &String) -> Path {
    let coordinate_strings: Vec<&str> = line.split(" -> ").collect();
    let mut path: Vec<Coordinate> = Vec::new();

    for coordinate_string in coordinate_strings {
        let parts: Vec<&str> = coordinate_string.split(",").collect();
        path.push(
            Coordinate { right: parts[0].parse().unwrap(),
                         down: parts[1].parse().unwrap()
                       }
        );
    }

    Path { path: path }
}

fn read_all_paths() -> Vec<Path> {
    let mut paths: Vec<Path> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        paths.push( read_path( &line.unwrap() ) );
    }

    paths
}

fn find_max_down(paths: &Vec<Path>) -> usize {
    let mut down: usize = 0;
    for path in paths {
        for coordinate in &path.path {
            if coordinate.down > down {
                down = coordinate.down;
            }
        }
    }

    down
}

fn find_max_right(paths: &Vec<Path>) -> usize {
    let mut right: usize = 0;
    for path in paths {
        for coordinate in &path.path {
            if coordinate.right > right {
                right = coordinate.right;
            }
        }
    }

    right
}

fn draw_one_path(tank: &mut matrix::Matrix<Tank>, path: &Path) {
    for i in 1..path.path.len() {
        let mystart = &path.path[i-1];
        let myend = &path.path[i];

        if mystart.right == myend.right {
            let mut begin = mystart.down;
            let mut done = myend.down;

            if mystart.down > myend.down {
                begin = myend.down;
                done = mystart.down;
            }

            for x in begin..=done {
                tank.set(mystart.right, x, Tank::Wall);
            }
        }

        if mystart.down == myend.down {
            let mut begin = mystart.right;
            let mut done = myend.right;

            if mystart.right > myend.right {
                begin = myend.right;
                done = mystart.right;
            }

            for x in begin..=done {
                tank.set(x, mystart.down, Tank::Wall);
            }
        }
    }
}

fn draw_all_paths(tank: &mut matrix::Matrix<Tank>, paths: &Vec<Path>) {
    for path in paths {
        draw_one_path(tank, &path);
    }

    tank.set(500, 0, Tank::Source);

    for x in 0..tank.width() {
        tank.set(x, tank.height() - 1, Tank::Wall);
    }
}

fn show_tank(tank: &matrix::Matrix<Tank>) {
    for down in 0..tank.height() {
        for right in 0..tank.width() {
            match tank.get(right, down) {
                Tank::Air => print!("."),
                Tank::Wall => print!("#"),
                Tank::Sand => print!("o"),
                Tank::Source => print!("+")
            }
        }
        println!("");
    }
}

fn drop_one_grain(tank: &mut matrix::Matrix<Tank>) -> bool {
    let mut x: usize = 500;
    let mut y: usize = 0;

    while y + 1 < tank.height() {
        if tank.get(x, y+1) == Tank::Air {
           // Do nothing 
        }
        else if tank.get(x-1, y+1) == Tank::Air {
            x = x - 1;
        }
        else if tank.get(x+1, y+1) == Tank::Air {
            x = x + 1;
        }
        else {
            break;
        }

        y += 1;
    }

    tank.set(x, y, Tank::Sand);
    x != 500 || y != 0
}

fn drop_many_grains(tank: &mut matrix::Matrix<Tank>) -> u32 {
    let mut grains_dropped: u32 = 0;

    while drop_one_grain(tank) {
        grains_dropped += 1;
    }

    grains_dropped + 1
}

fn main() {
    let paths = read_all_paths();
    let mut tank: matrix::Matrix<Tank> = 
        matrix::build( find_max_right(&paths) * 2, find_max_down(&paths) + 3, Tank::Air);

    draw_all_paths(&mut tank, &paths);
    let grains_dropped = drop_many_grains(&mut tank);
    show_tank(&tank);
    println!("{}", grains_dropped);
}
