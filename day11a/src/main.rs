use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

type Octopuses = [[u32; 10]; 10];

fn neighbors(p: (usize, usize)) -> Vec<(usize, usize)> {
    (-1isize..=1)
        .map(move |w| (-1isize..=1).map(move |h| (w, h)))
        .flatten() // Al the adjacent (x,y) coordinates
        .map(|offset| {
            (
                (p.0 as isize + offset.0) as usize,
                (p.1 as isize + offset.1) as usize,
            )
        })
        .filter(|x| { //check the bounds
            (x.0 != p.0 || x.1 != p.1) && (0..10).contains(&x.0) && (0..10).contains(&x.1)
        })
        .collect()
}

fn step(o: Octopuses) -> (u32, Octopuses) {
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut to_flash: VecDeque<(usize, usize)> = VecDeque::new();
    let mut ret = o;

    // Update by 1
    for (x, y) in (0..10).map(move |w| (0..10).map(move |h| (w, h))).flatten() {
        ret[y][x] += 1;

        if ret[y][x] > 9 {
            to_flash.push_back((x, y));
        }
    }

    while to_flash.len() > 0 {
        let f = to_flash.pop_front().unwrap();
        if flashed.contains(&f) {
            continue;
        }
        flashed.insert(f);

        for n in neighbors(f) {
            ret[n.1][n.0] += 1;
            if ret[n.1][n.0] > 9 {
                to_flash.push_back(n);
            };
        }
    }

    // Set those that flashed to 0;
    flashed.iter().for_each(|f| ret[f.1][f.0] = 0);

    (flashed.len() as u32, ret)
}

fn main() {
    let stdin = io::stdin();
    let mut state: Octopuses = [[0; 10]; 10];

    for (i, line) in stdin.lock().lines().enumerate().take(10) {
        state[i] = <[u32; 10]>::try_from(
            line.unwrap()
                .chars()
                .map(|x| (x as u32) - '0' as u32)
                .collect::<Vec<u32>>(),
        )
        .unwrap();
    }

    let mut flash_total = 0;
    for _ in 0..100 {
        let r = step(state);
        flash_total += r.0;
        state = r.1;
    }
    println!("{}", flash_total);
}
