use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next();

    let fish_school: Vec<usize> = line.unwrap().unwrap().split(',').map(|x| x.parse().unwrap()).collect();
    let mut fish: [u64; 7] = [0; 7];
    let (mut fish7, mut fish8) = (0u64, 0u64);

    for f in fish_school {
        fish[f] += 1
    }

    for today in 0..=256 {
        let yesterdays_zero = (today + 6) % 7;
        let day6 = yesterdays_zero;
        let newborn_fish = fish[yesterdays_zero];

        // young fish age slowly
        fish[day6] += fish7;
        fish7 = fish8;
        fish8 = newborn_fish;
    }

    println!("{}", fish.iter().sum::<u64>() + fish8 + fish7);
}
