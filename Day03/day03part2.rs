use std::io;
use std::io::prelude::*;

struct Rucksack {
    pack: Vec<u8>
}

fn read_rucksack(line: String) -> Rucksack {
    let mut mypack: Vec<u8> = Vec::new();

    for ch in line.as_bytes() {
        mypack.push(*ch);
    }

    mypack.sort();

    Rucksack {
        pack: mypack
    }
}

fn read_all_rucksacks() -> Vec< Rucksack > {
    let mut rucksacks: Vec< Rucksack > = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        rucksacks.push( read_rucksack(line.unwrap()) );
    }

    rucksacks
}

fn binary_search(needle: u8, haystack: &Vec<u8>) -> bool {
    let mut low: usize = 0;
    let mut high: usize = haystack.len() - 1;

    while low <= high {
        let mid: usize = (low + high) / 2;
        if haystack[mid] < needle {
            low = mid + 1;
        }
        else if mid > 0 && haystack[mid] > needle {
            high = mid - 1;
        }
        else if mid == 0 && haystack[mid] > needle {
            return false;
        }
        else {
            return true;
        }
    }

    false
}

fn get_common_item_in_group(first: &Rucksack, second: &Rucksack, third: &Rucksack) -> u8 {
    for item in &first.pack {
        if binary_search(*item, &second.pack) && binary_search(*item, &third.pack) {
            return *item;
        }
    }
    0
}

fn get_common_item_in_all_groups(rucksacks: &Vec<Rucksack>) -> Vec<u8> {
    let mut group_items: Vec<u8> = Vec::new();
    let mut i: usize = 0;

    while i < rucksacks.len() {
        group_items.push( get_common_item_in_group(&rucksacks[i], &rucksacks[i+1], &rucksacks[i+2]) );
        i += 3;
    }

    group_items
}

fn get_priority_value(value: u8) -> u32 {
    if 65 <= value && value <= 90 {
        return ((value - 65) + 27).into();
    }

    (value - 96).into()
}

fn get_total_of_all_groups(group_items: &Vec<u8>) -> u32 {
    let mut total: u32 = 0;
    for group_item in group_items {
        let priority = get_priority_value(*group_item);
        total += priority;
    }
    total
}

fn main() {
    let rucksacks = read_all_rucksacks();
    let group_items = get_common_item_in_all_groups(&rucksacks);
    println!("{}", get_total_of_all_groups(&group_items));
}
