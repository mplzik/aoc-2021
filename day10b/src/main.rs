use std::collections::HashMap;
use std::io::{self, BufRead};
use Bracket::*;
use Symbol::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Symbol {
    Opening(Bracket),
    Closing(Bracket),
}

fn symbol_from_char(c: char) -> Option<Symbol> {
    let pos = ['(', '[', '{', '<', ')', ']', '}', '>']
        .iter()
        .position(|&x| x == c)?;
    let brackets = [Round, Square, Curly, Angle];

    if pos > 3 {
        Some(Closing(brackets[pos - 4]))
    } else {
        Some(Opening(brackets[pos]))
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scores: Vec<u64> = Vec::new();
    let score = HashMap::from([(Round, 1u64), (Square, 2u64), (Curly, 3u64), (Angle, 4u64)]);

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let mut stack: Vec<Bracket> = Vec::new();
        let mut corrupted = false;

        for in_char in line.chars() {
            let input = symbol_from_char(in_char).unwrap();
            match input {
                Opening(b) => stack.push(b),
                Closing(b) => {
                    match stack.last() {
                        Some(top_symbol) if b == *top_symbol => {
                            stack.pop();
                        }
                        _ => { // Anything else is corrupted line.
                            corrupted = true;
                            break;
                        }
                    }
                }
            }
        }
        if corrupted || stack.len() == 0 {
            continue;
        };
        let val = stack
            .iter()
            .rev()
            .map(|x| score[x])
            .fold(0, |a, x| 5 * a + x);
        scores.push(val);
    }

    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}
