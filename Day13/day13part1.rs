use std::io;
use std::io::prelude::*;

enum ListType {
    Number(i32),
    List(Vec<ListType>)
}

struct Pair {
    left: Vec<ListType>,
    right: Vec<ListType>
}

fn read_all_lines() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines.push( line.unwrap() );
    }

    lines
}

fn read_all_pairs(lines: &Vec<String>) -> Vec<Pair> {
    let mut i: usize = 0;
    let mut pairs: Vec<Pair> = Vec::new();

    while i < lines.len() {
        let mut left: usize = 0;
        let mut right: usize = 0;

        pairs.push( Pair { left: read_list(&lines[i], &mut left), right: read_list(&lines[i+1], &mut right) } );
        i += 3;
    }

    pairs
}

fn read_list(line: &String, i: &mut usize) -> Vec<ListType> {
    *i += 1;
    let mut mylist: Vec<ListType> = Vec::new();
    let chars: Vec<char> = line.chars().collect();

    while *i < chars.len() && chars[*i] != ']' {
        if chars[*i] == '[' {
            mylist.push( ListType::List(read_list(line, i)) );
        }
        else {
            mylist.push( ListType::Number(read_number(line, i)) );
        }

        while *i < chars.len() && chars[*i] == ',' {
            *i += 1;
        }
    }

    *i += 1;

    if *i < chars.len() && chars[*i] == ',' {
        *i += 1;
    }
    mylist
}

fn read_number(line: &String, i: &mut usize) -> i32 {
    let mut num_string: String = String::from("");
    let chars: Vec<char> = line.chars().collect();

    while chars[*i] != ',' && chars[*i] != ']' {
        num_string.push(chars[*i]);
        *i += 1;
    }

    let value: i32 = num_string.trim().parse().expect("Please type a number!");
    value
}

fn compare_lists(left: &Vec<ListType>, right: &Vec<ListType>) -> Option<bool> {
    let mut index: usize = 0;

    while index < left.len() && index < right.len() { 
        match (&left[index], &right[index]) {
            (ListType::Number(x), ListType::Number(y)) => {
                if x < y {
                    return Some(true);
                }
                if y < x {
                    return Some(false);
                }
            }

            (ListType::List(x), ListType::List(y)) => {
                let comparison = compare_lists(&x, &y);
                match comparison {
                    Some(x) => { return Some(x); }
                    None => { }
                }
            }

            (ListType::List(x), ListType::Number(y)) => {
                let z = vec![ListType::Number(*y)];
                let comparison = compare_lists(&x, &z);
                match comparison {
                    Some(x) => { return Some(x); }
                    None => { }
                }
            }

            (ListType::Number(x), ListType::List(y)) => {
                let z = vec![ListType::Number(*x)];
                let comparison = compare_lists(&z, &y);
                match comparison {
                    Some(x) => { return Some(x); }
                    None => { }
                }
            }
        }
        index += 1;
    }

    if index >= left.len() && index < right.len() {
        return Some(true);
    }

    if index < left.len() && index >= right.len() {
        return Some(false);
    }

    None
}

fn is_left_better_than_right(left: &Vec<ListType>, right: &Vec<ListType>) -> bool {
    compare_lists(left, right).unwrap()
}

fn main() {
    let lines = read_all_lines();
    let pairs = read_all_pairs(&lines);

    let mut sum: i32 = 0;
    let mut index: i32 = 1;
    for pair in pairs {
        if is_left_better_than_right(&pair.left, &pair.right) {
            sum += index;
        }
        index += 1;
    }
    println!("{}", sum);
}
