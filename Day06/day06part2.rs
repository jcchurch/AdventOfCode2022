use std::io;
use std::collections::VecDeque;

fn read_signal() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line
}

fn is_unique(message: &VecDeque<char>) -> bool {
    let mut tally: u32 = 0;

    for character in message.iter() {
        let mask: u32 = 1 << (*character as u32 - 'a' as u32);
        if tally & mask > 0 {
            return false;
        }

        tally |= mask;
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
    0
}

fn main() {
    let signal = read_signal();
    println!("{}", 1+find_signal_message_start(&signal));
}
