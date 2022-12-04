use std::io;
use std::io::prelude::*;

// A for Rock, B for Paper, and C for Scissors
// X for Rock, Y for Paper, and Z for Scissors
// 1 for Rock, 2 for Paper, and 3 for Scissors
// 0 if you lost, 3 if the round was a draw, and 6 if you won
//
// Update for Part 2
// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.

struct TTTRound {
    opponent: char,
    you: char
}

impl TTTRound {

    #[allow(dead_code)]
    fn get_score(&self) -> i32 {
        let mut score = 0;
        if self.you == 'X' { score = 1; }
        if self.you == 'Y' { score = 2; }
        if self.you == 'Z' { score = 3; }

        if (self.opponent == 'A' && self.you == 'X') ||
           (self.opponent == 'B' && self.you == 'Y') ||
           (self.opponent == 'C' && self.you == 'Z') {
               return 3 + score;
        }


        if (self.opponent == 'A' && self.you == 'Y') ||
           (self.opponent == 'B' && self.you == 'Z') ||
           (self.opponent == 'C' && self.you == 'X') {
               return 6 + score;
        }

        return score;
    }

    fn get_strategy_score(&self) -> i32 {
        if self.opponent == 'A' && self.you == 'X' {
            return 3; // Return scissors
        }
        if self.opponent == 'B' && self.you == 'X' {
            return 1; // Return rock
        }
        if self.opponent == 'C' && self.you == 'X' {
            return 2; // Return paper
        }

        if self.opponent == 'A' && self.you == 'Y' {
            return 1 + 3; // Return rock
        }
        if self.opponent == 'B' && self.you == 'Y' {
            return 2 + 3; // Return paper
        }
        if self.opponent == 'C' && self.you == 'Y' {
            return 3 + 3; // Return scissors
        }

        if self.opponent == 'A' && self.you == 'Z' {
            return 2 + 6; // Return paper
        }
        if self.opponent == 'B' && self.you == 'Z' {
            return 3 + 6; // Return scissors
        }
        if self.opponent == 'C' && self.you == 'Z' {
            return 1 + 6; // Return rock
        }

        return 0;
    }
}

fn read_round(line: String) -> TTTRound {
    TTTRound {
        opponent: line.chars().nth(0).unwrap(),
        you: line.chars().nth(2).unwrap()
    }
}

fn read_all_rounds() -> Vec< TTTRound > {
    let mut rounds: Vec< TTTRound > = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        rounds.push( read_round(line.unwrap()) );
    }

    rounds
}

fn get_sum_of_all_rounds(rounds: &Vec<TTTRound>) -> i32 {
    let mut total: i32 = 0;
    for round in rounds {
        total += round.get_strategy_score();
    }
    total
}

fn main() {
    let rounds = read_all_rounds();    
    println!("{}", get_sum_of_all_rounds(&rounds));
}