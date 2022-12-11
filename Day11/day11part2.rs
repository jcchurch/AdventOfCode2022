use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

struct Monkey {
    id: i32,
    items: VecDeque<i32>,
    operation: String,
    test: i32,
    true_branch: usize,
    false_branch: usize,
    inspections: usize
}

fn read_monkey(lines: &Vec<String>, i: &mut usize) -> Monkey {
    let mut id: i32 = -1;
    let mut items: VecDeque<i32> = VecDeque::new();
    let mut operation: String = "noop".to_string();
    let mut test: i32 = -1;
    let mut true_branch: usize = 0;
    let mut false_branch: usize = 0;

    while *i < lines.len() && lines[*i] != "" {
        let parts: Vec<&str> = lines[*i].split_whitespace().collect();

        if parts[0] == "Monkey" {
            let idparts: Vec<&str> = parts[1].split(':').collect();
            id = idparts[0].parse().unwrap();
        }

        if parts[0] == "Starting" {
            let allitems: Vec<&str> = lines[*i].split(": ").collect();
            let itemparts: Vec<&str> = allitems[1].split(", ").collect();
            for item in itemparts {
                items.push_back( item.parse().unwrap() );
            }
        }

        if parts[0] == "Operation:" {
            let operation_parts: Vec<&str> = lines[*i].split(" = ").collect();
            operation = operation_parts[1].to_string();
        }
        
        if parts[0] == "Test:" {
            test = parts[3].parse().unwrap();
        }

        if parts[0] == "If" && parts[1] == "true:" {
            true_branch = parts[5].parse().unwrap();
        }

        if parts[0] == "If" && parts[1] == "false:" {
            false_branch = parts[5].parse().unwrap();
        }

        *i += 1;
    }

    while *i < lines.len() && lines[*i] == "" {
        *i += 1;
    }

    Monkey {
        id: id,
        items: items,
        operation: operation,
        test: test,
        true_branch: true_branch,
        false_branch: false_branch,
        inspections: 0
    }
}

fn read_all_lines() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines.push( line.unwrap() );
    }

    lines
}

fn read_all_monkeys(lines: &Vec<String>) -> Vec<Monkey> {
    let mut i: usize = 0;
    let mut monkeys: Vec<Monkey> = Vec::new();

    while i < lines.len() {
        monkeys.push( read_monkey(&lines, &mut i) );
    }

    monkeys
}

fn process_monkey(monkeys: &mut Vec<Monkey>, monkey_id: usize, megamod: i32) {
    while monkeys[monkey_id].items.len() > 0 {
        monkeys[monkey_id].inspections += 1;

        let item: i32 = monkeys[monkey_id].items.pop_front().unwrap();
        let operation_parts: Vec<&str> = monkeys[monkey_id].operation.split(' ').collect();
        let mut next_item: i32 = 0;
        let mut a: i32;
        let mut b: i32;

        // Inspect
        if operation_parts[0] == "old" {
            a = item;
        }
        else {
            a = operation_parts[0].parse().unwrap();
        }

        if operation_parts[2] == "old" {
            b = item;
        }
        else {
            b = operation_parts[2].parse().unwrap();
        }

        a %= megamod;

        if operation_parts[1] == "+" {
            next_item = a + b;
        }
        else if operation_parts[1] == "*" {
            next_item = a * b;
        }

        next_item %= megamod;

        // Test and Throw
        let mut next_monkey = monkeys[monkey_id].false_branch;
        if next_item % monkeys[monkey_id].test == 0 {
            next_monkey = monkeys[monkey_id].true_branch;
        }
        monkeys[ next_monkey ].items.push_back(next_item);
    }
}

fn process_one_round(monkeys: &mut Vec<Monkey>, megamod: i32) {
    for monkey_id in 0..monkeys.len() {
        process_monkey(monkeys, monkey_id, megamod);
    }
}

fn compute_monkey_business(monkeys: &Vec<Monkey>) -> usize {
    let mut inspections: Vec<usize> = Vec::new();
    for monkey in monkeys {
        inspections.push( monkey.inspections );
    }

    inspections.sort();

    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

fn show_monkey(monkey: &Monkey) {
    print!("Monkey {:2}: ", monkey.id);
    for item in &monkey.items {
        print!("{}, ", item);
    }
    println!("");
}

fn find_mega_mod(monkeys: &Vec<Monkey>) -> i32 {
    let mut megamod = 1;
    for monkey in monkeys {
        megamod *= monkey.test;
    }
    megamod
}

fn main() {
    let lines = read_all_lines();
    let mut monkeys = read_all_monkeys(&lines);
    let megamod = find_mega_mod(&monkeys);

    for _ in 0..20 {
        process_one_round(&mut monkeys, megamod);
    }

    println!("{}", compute_monkey_business(&monkeys));
}
