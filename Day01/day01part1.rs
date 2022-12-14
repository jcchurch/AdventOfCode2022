use std::io;
use std::io::prelude::*;

fn read_calories(line: String) -> i32 {
    if line == "" { return 0; }

    let calories: i32 =
        line
        .trim()
        .parse()
        .expect("Please type a number!");

    calories
}

fn read_all_elves() -> Vec< Vec<i32> > {
    let mut all_elves: Vec< Vec<i32> > = Vec::new();
    let mut elf: Vec<i32> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let calories = read_calories(line.unwrap());

        if calories == 0 {
            all_elves.push(elf.clone());
            elf.clear();
        }
        else {
            elf.push(calories);
        }
    }

    all_elves
}

fn sum_of_elf(elf: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for calories in elf {
        sum += calories;
    }
    sum
}

fn largest_calories_among_elves(all_elves: &Vec< Vec<i32> >) -> i32 {
   let mut largest = -1; 
   for elf in all_elves {
       let total = sum_of_elf(&elf);
       if total > largest {
           largest = total;
       }
   }
   largest
}

fn main() {
    let all_elves = read_all_elves();    
    println!("{}", largest_calories_among_elves(&all_elves));
}
