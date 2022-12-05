use std::io;
use std::io::prelude::*;

struct Workorder {
    first_start: u32,
    first_end: u32, 
    second_start: u32,
    second_end: u32
}

fn read_workorder(line: String) -> Workorder {
    let ranges: Vec<&str> = line.split(",").collect();
    let first: Vec<&str> = ranges[0].split("-").collect();
    let second: Vec<&str> = ranges[1].split("-").collect();

    Workorder {
        first_start:  first[0].parse().unwrap(),
        first_end:    first[1].parse().unwrap(),
        second_start: second[0].parse().unwrap(),
        second_end:   second[1].parse().unwrap()
    }
}

fn read_all_workorders() -> Vec<Workorder> {
    let mut workorders: Vec<Workorder> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        workorders.push( read_workorder(line.unwrap()) );
    }

    workorders
}

fn is_workorder_full_contained(workorder: &Workorder) -> bool {
    if workorder.first_start <= workorder.second_start && workorder.second_end <= workorder.first_end {
        return true;
    }

    if workorder.second_start <= workorder.first_start && workorder.first_end <= workorder.second_end {
        return true;
    }

    return false;
}

fn count_fully_contained_workorders(workorders: &Vec<Workorder>) -> u32 {
    let mut count = 0;
    for workorder in workorders {
        if is_workorder_full_contained(&workorder) {
            count += 1;
        }
    }
    count
}

fn main() {
    let workorders = read_all_workorders();
    println!("{}", count_fully_contained_workorders(&workorders));
}
