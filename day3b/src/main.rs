use std::io::{self, BufRead};

fn countones(data: &Vec<u32>, bit: u32) -> usize {
    data.iter().filter(|x| *x & (1 << bit) != 0).count()
}

fn main() {
    let stdin = io::stdin();
    let mut values: Vec<u32> = Vec::new();
    let mut width = 0;

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        values.push(u32::from_str_radix(&line, 2).unwrap());
        width = line.len()
    }

    let mut oxygen_candidates = values.clone();
    let mut co2scrub_candidates = values.clone();

    for i in (0..width).rev() {
        let mask = 1 << i;
        if oxygen_candidates.len() > 1 {
            let ones = countones(&oxygen_candidates, i as u32);
            oxygen_candidates = oxygen_candidates
                .iter()
                .filter(|x| {
                    if ones >= oxygen_candidates.len() - ones {
                        *x & mask != 0
                    } else {
                        *x & mask == 0
                    }
                })
                .cloned()
                .collect();
        };
        if co2scrub_candidates.len() > 1 {
            let ones = countones(&co2scrub_candidates, i as u32);
            co2scrub_candidates = co2scrub_candidates
                .iter()
                .filter(|x| {
                    if ones < co2scrub_candidates.len() - ones {
                        *x & mask != 0
                    } else {
                        *x & mask == 0
                    }
                })
                .cloned()
                .collect();
        };
    }

    println!("{} {}", oxygen_candidates[0], co2scrub_candidates[0]);
    println!("{}", oxygen_candidates[0] * co2scrub_candidates[0]);
}
