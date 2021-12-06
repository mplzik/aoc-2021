use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut i = 0;
    let mut depths = [u32::MAX; 4];
    let mut count = 0;

    for line in stdin.lock().lines() {
        let now: u32 = line.unwrap().parse().unwrap();
        depths[i] = now;
        if depths[i] > depths[(i+1) % 4] {
            count += 1;
        }
        i = (i + 1) % 4;
    }

    println!("{}", count);
}
