use std::io::{self, BufRead};

fn fish(counter: u32, remaining: u32) -> u32 {
    match counter {
        0 if remaining > 0 => { 
            fish(7, remaining - counter) + fish(9, remaining - counter)
        },
        x if x < remaining => {
            fish(0, remaining - counter)
        },
        x if x >= remaining => {
            1
        }
        _ => panic!("Unexpected case: fish({}, {})!", counter, remaining)
    }

}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next();
    let fish_school: Vec<u32> = line.unwrap().unwrap().split(',').map(|x| x.parse().unwrap()).collect();

    println!("{}", fish_school.iter().map(|&x| fish(x, 80)).sum::<u32>() );
}
