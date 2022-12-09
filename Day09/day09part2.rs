use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::cmp;

struct Motion {
    direction: char,
    distance: u32
}

struct Point { x: i32, y: i32 }

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
    let mut knots: Vec<Point> = Vec::new();
    for _ in 0..10 {
        knots.push( Point { x: 0, y: 0 } );
    }

    let mut set: HashSet<String> = HashSet::new();
    set.insert("0:0".to_string());

    for motion in motions {
        execute_one_motion(&motion, &mut knots, &mut set);
    }

    set
}

fn execute_one_motion(motion: &Motion, knots: &mut Vec<Point>, set: &mut HashSet<String>) {
    for _ in 0..motion.distance {
        if motion.direction == 'R' { knots[0].x += 1; }
        if motion.direction == 'L' { knots[0].x -= 1; }
        if motion.direction == 'U' { knots[0].y += 1; }
        if motion.direction == 'D' { knots[0].y -= 1; }

        for i in 1..10 {
            let distance = compute_distance(knots[i-1].x, knots[i-1].y, knots[i].x, knots[i].y);

            if distance > 1 {
                let dirx = (knots[i-1].x - knots[i].x).signum();
                let diry = (knots[i-1].y - knots[i].y).signum();
                knots[i].x += dirx;
                knots[i].y += diry;
                if i == 9 {
                    set.insert( knots[i].x.to_string() + &String::from(":") + &knots[i].y.to_string() );
                }
            }
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
