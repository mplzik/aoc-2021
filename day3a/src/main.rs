use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut counts: Vec<u32> = Vec::new();
    let mut lines = 0;
    let (mut gamma, mut epsilon) = (0, 0);

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        counts.resize(line.len(), 0);
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                counts[i] += 1;
            };
        }

        lines += 1;
    }

    let mut m = 1;
    for i in (0..counts.len()).rev() {
        if (counts[i] > lines - counts[i]) {
            gamma += m
        } else {
            epsilon += m
        }
        m *= 2;
    }

    println!("{}", gamma * epsilon);
}
