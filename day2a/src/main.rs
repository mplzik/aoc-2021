use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let (mut x, mut y) = (0, 0);

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let s: Vec<&str> = line.split(' ').collect();

        match (s[0], s[1].parse::<u32>().unwrap()) {
            ("forward", v) => x += v,
            ("up", v) => y -= v,
            ("down", v) => y += v,
            (_, _) => {
                panic!("Unknown command {}", line);
            }
        }
    }

    println!("{}", x * y);
}
