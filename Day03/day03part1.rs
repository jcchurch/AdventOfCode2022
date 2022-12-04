use std::io;
use std::io::prelude::*;

struct Rucksack {
    first_half: String,
    second_half: String
}

fn read_rucksack(line: String) -> Rucksack {
    let length: usize = line.len() / 2;
    Rucksack {
        first_half: line[..length].to_string(),
        second_half: line[length..(length*2)].to_string()
    }
}

fn read_all_rucksacks() -> Vec< Rucksack > {
    let mut rucksacks: Vec< Rucksack > = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let rucksack = read_rucksack(line.unwrap());
        rucksacks.push( rucksack );
    }

    rucksacks
}

fn get_letter_in_both_halfs(rucksack: &Rucksack) -> char {
    for first in rucksack.first_half.chars() {
        for second in rucksack.second_half.chars() {
            if first == second {
                return first;
            }
        }
    }

    '?'
}

fn get_priority_value(item: &char) -> u32 {
    let value = *item as u32;
    if 65 <= value && value <= 90 {
        return (value - 65) + 27;
    }

    return value - 96;
}

fn get_common_item_in_all_rucksacks(rucksacks: &Vec<Rucksack>) -> Vec<char> {
    let mut priority_items: Vec<char> = Vec::new();
    for rucksack in rucksacks {
        priority_items.push( get_letter_in_both_halfs(&rucksack) );
    }
    priority_items
}

fn get_total_of_all_rucksacks(priority_items: &Vec<char>) -> u32 {
    let mut total: u32 = 0;
    for priority_item in priority_items {
        total += get_priority_value(&priority_item);
    }
    total
}

fn main() {
    let rucksacks = read_all_rucksacks();
    let priority_items = get_common_item_in_all_rucksacks(&rucksacks);
    println!("{}", get_total_of_all_rucksacks(&priority_items));
}
