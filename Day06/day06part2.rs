use std::io;
use std::collections::VecDeque;

fn read_signal() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line
}

fn is_unique(message: &VecDeque<char>) -> bool {
    let mut tally: Vec<u32> = Vec::new();

    for _ in 0..26 {
        tally.push(0);
    }

    for character in message.iter() {
        if tally[*character as usize - 'a' as usize] > 0 {
            return false;
        }

        tally[*character as usize - 'a' as usize] += 1;
    }

    true
}

fn find_signal_message_start(signal: &String) -> usize {
    let chars: Vec<char> = signal.chars().collect();
    let mut message: VecDeque<char> = VecDeque::new();

    for i in 0..13 {
        message.push_back(chars[i]);
    }

    for i in 13..signal.len() {
        message.push_back(chars[i]);
        if is_unique(&message) {
            return i;
        }
        message.pop_front();
    }
    1000
}

fn main() {
    let signal = read_signal();
    println!("{}", 1+find_signal_message_start(&signal));
}