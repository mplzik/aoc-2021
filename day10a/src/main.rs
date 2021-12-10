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
    let mut s = 0;
    let score = HashMap::from([(Round, 3), (Square, 57), (Curly, 1197), (Angle, 25137)]);

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let mut stack: Vec<Bracket> = Vec::new();

        for in_char in line.chars() {
            let input = symbol_from_char(in_char).unwrap();
            match input {
                Opening(b) => stack.push(b),
                Closing(b) => {
                    match stack.last() {
                        Some(top_symbol) if b == *top_symbol => {
                            stack.pop();
                        }
                        _ => {
                            // Anything else is corrupted line.
                            s += score[&b];
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("{}", s);
}
