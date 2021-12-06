use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut prev: u32 = u32::MAX;
    let mut count = 0;

    for line in stdin.lock().lines() {
        let now: u32 = line.unwrap().parse().unwrap();
        if now > prev {
            count += 1;
        }
        prev = now;
    }

    println!("{}", count);
}
