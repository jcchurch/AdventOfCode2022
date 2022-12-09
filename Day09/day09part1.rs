use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::cmp;

struct Motion {
    direction: char,
    distance: u32
}

fn read_motion(line: String) -> Motion {
    let parts: Vec<&str> = line.split_whitespace().collect();

    Motion {
        direction:  line.chars().nth(0).unwrap(),
        distance:   parts[1].parse().unwrap()
    }
}

fn read_all_motions() -> Vec<Motion> {
    let mut motions: Vec<Motion> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        motions.push( read_motion(line.unwrap()) );
    }

    motions
}

fn execute_motions(motions: &Vec<Motion>) -> HashSet<String> {
    let mut headx: i32 = 0;
    let mut heady: i32 = 0;
    let mut tailx: i32 = 0;
    let mut taily: i32 = 0;
    let mut set: HashSet<String> = HashSet::new();
    set.insert("0:0".to_string());

    for motion in motions {
        execute_one_motion(&motion, &mut headx, &mut heady, &mut tailx, &mut taily, &mut set);
    }

    set
}

fn execute_one_motion(motion: &Motion, headx: &mut i32, heady: &mut i32, tailx: &mut i32, taily: &mut i32, set: &mut HashSet<String>) {
    for _ in 0..motion.distance {
        if motion.direction == 'R' { *headx += 1; }
        if motion.direction == 'L' { *headx -= 1; }
        if motion.direction == 'U' { *heady += 1; }
        if motion.direction == 'D' { *heady -= 1; }

        let distance = compute_distance(*headx, *heady, *tailx, *taily);

        if distance > 1 {
            *tailx += (*headx - *tailx).signum();
            *taily += (*heady - *taily).signum();
            set.insert( tailx.to_string() + &String::from(":") + &taily.to_string() );
        }
    }
}

fn compute_distance(headx: i32, heady: i32, tailx: i32, taily: i32) -> i32 {
    cmp::max( (headx-tailx).abs(), (heady-taily).abs() )
}

fn main() {
    let motions = read_all_motions();
    let set = execute_motions(&motions);
    println!("{}", set.len());
}
