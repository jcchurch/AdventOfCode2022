use std::io;

fn read_signal() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line
}

fn find_signal_start(signal: &String) -> usize {
    let chars: Vec<char> = signal.chars().collect();
    for i in 3..signal.len() {
        if chars[i-3] != chars[i-2] &&
           chars[i-3] != chars[i-1] &&
           chars[i-3] != chars[i]   &&
           chars[i-2] != chars[i-1] &&
           chars[i-2] != chars[i]   &&
           chars[i-1] != chars[i] {
            return i;
        }
    }
    1000
}

fn main() {
    let signal = read_signal();
    println!("{}", 1+find_signal_start(&signal));
}
