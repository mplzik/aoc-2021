use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let pos_str = stdin.lock().lines().next().unwrap().unwrap();
    let pos: Vec<i32> = pos_str.split(',').map(|x| x.parse().unwrap()).collect();

    let min = *pos.iter().min().unwrap();
    let max = *pos.iter().max().unwrap();

    let mut minfuel = i32::MAX;
    for hpos in min..=max {
        let fuel = pos
            .iter()
            .map(|x| (x - hpos).abs() * ((x - hpos).abs() + 1) / 2)
            .sum();
        if fuel < minfuel {
            minfuel = fuel
        };
    }

    println!("{}", minfuel);
}
