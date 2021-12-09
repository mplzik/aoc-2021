use std::io::{self, BufRead};

fn sample(map: &Vec<Vec<u8>>, x:isize, y:isize) -> u32 {
    if y < 0 || y >= map.len() as isize{
        return u32::MAX;
    }

    let line = &map[y as usize];
    if x < 0 || x >= line.len() as isize {
        return u32::MAX;
    }

    map[y as usize][x as usize] as u32
}

fn main() { 
    let stdin = io::stdin();
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut s = 0;

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let heights: Vec<u8> = line.chars().map(|x| (x.to_string().parse().unwrap()) ).collect();
        map.push(heights)
    }

    for y in 0.. map.len() {
        let line = &map[y];
        for x in 0..line.len() {
            let c = map[y][x] as u32;
            let neighbors: [[isize;2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];
            
            let res = neighbors.iter().map(|n| c < sample(&map, n[0]+x as isize, n[1] + y as isize)).all(|x| x == true);
            if res {
                s += c + 1;
            } 
        }
    }

    println!("{}", s);
}
