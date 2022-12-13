use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;

enum ListType {
    Number(i32),
    List(Vec<ListType>)
}

struct Packet {
    line: String,
    list: Vec<ListType>
}

fn read_all_lines() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines.push( line.unwrap() );
    }

    lines
}

fn read_all_packets(lines: &Vec<String>) -> Vec<Packet> {
    let mut packets: Vec<Packet> = Vec::new();

    for line in lines {
        if line != "" {
            let mut index: usize = 0;
            packets.push( Packet { line: line.to_string(), list: read_list(&line, &mut index) } );
        }
    }

    packets
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

fn compare_lists(left: &Vec<ListType>, right: &Vec<ListType>) -> Ordering {
    let mut index: usize = 0;

    while index < left.len() && index < right.len() { 
        match (&left[index], &right[index]) {
            (ListType::Number(x), ListType::Number(y)) => {
                if x < y {
                    return Ordering::Less;
                }
                if y < x {
                    return Ordering::Greater;
                }
            }

            (ListType::List(x), ListType::List(y)) => {
                let comparison = compare_lists(&x, &y);
                match comparison {
                    Ordering::Less => { return Ordering::Less; }
                    Ordering::Greater => { return Ordering::Greater; }
                    Ordering::Equal => { }
                }
            }

            (ListType::List(x), ListType::Number(y)) => {
                let z = vec![ListType::Number(*y)];
                let comparison = compare_lists(&x, &z);
                match comparison {
                    Ordering::Less => { return Ordering::Less; }
                    Ordering::Greater => { return Ordering::Greater; }
                    Ordering::Equal => { }
                }
            }

            (ListType::Number(x), ListType::List(y)) => {
                let z = vec![ListType::Number(*x)];
                let comparison = compare_lists(&z, &y);
                match comparison {
                    Ordering::Less => { return Ordering::Less; }
                    Ordering::Greater => { return Ordering::Greater; }
                    Ordering::Equal => { }
                }
            }
        }
        index += 1;
    }

    if index >= left.len() && index < right.len() {
        return Ordering::Less;
    }

    if index < left.len() && index >= right.len() {
        return Ordering::Greater;
    }

    Ordering::Equal
}

fn is_left_less_than_right(left: &Packet, right: &Packet) -> Ordering {
    compare_lists(&left.list, &right.list)
}

fn main() {
    let mut lines = read_all_lines();
    lines.push("[[2]]".to_string());
    lines.push("[[6]]".to_string());

    let mut packets = read_all_packets(&lines);

    packets.sort_by( |a, b| is_left_less_than_right(&a, &b) );

    let mut two = 0;
    let mut six = 0;
    let mut index = 1;
    for packet in packets {
        if packet.line == "[[2]]" {
            two = index;
        }
        if packet.line == "[[6]]" {
            six = index;
        }
        index += 1;
    }
    println!("{}", two * six);
}
