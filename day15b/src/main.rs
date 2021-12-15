use std::io::{self, BufRead};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

type Point = [usize; 2];

fn neighbors(p: Point, w: usize, h: usize) -> Vec<Point> {
    [[0, 1], [0, -1], [1, 0], [-1, 0]].iter()
        .map(|o| [(p[0] as isize + o[0]) as usize, (p[1] as isize + o[1]) as usize])
        .filter(|n| (0..w).contains(&n[0]) && (0..h).contains(&n[1]))
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut map: HashMap<Point, u32> = HashMap::new();
    let mut distances: HashMap<Point, u32> = HashMap::new();
    let (mut w, mut h) = (0usize, 0usize);

    for (y, line) in stdin.lock().lines().enumerate() {
        if h < (y + 1) { h = y + 1 };
        for (x, c) in line.unwrap().chars().enumerate() {
            if w < (x + 1) { w = x + 1 };
            map.insert([x, y], c as u32 - '0' as u32);
        }
    }

    // Preprocess the map
    for y in 0..h {
        for x in 0..w {
            let val = *map.get(&[x, y]).unwrap();

            for oy in 0..5 {
                for ox in 0..5 {
                    if ox == 0 && oy == 0 {
                        continue;
                    }
                    let newpos = [x + w*ox, y + h*oy];
                    let offset = (ox as isize + oy as isize + 10) % 10; 
                    let mut newval = offset + val as isize;
                    if newval > 9 {
                        newval -= 9;
                    }
                    map.insert(newpos, newval as u32);

                }
            }
        }
    }

    w *= 5;
    h *= 5;


    let pos: Point = [0, 0];
    let mut heap = BinaryHeap::new();

    heap.push(Reverse((0, pos)));
    distances.insert(pos, 0);

    while heap.len() > 0 {
        let (dist, pos) = heap.pop().unwrap().0;

        for neighbor_pos in neighbors(pos, w, h) {
            let neighbor_val = map.get(&neighbor_pos).unwrap();
            let neighbor_dist = distances.entry(neighbor_pos).or_insert(u32::MAX);

            if dist + *neighbor_val < *neighbor_dist {
                *neighbor_dist = dist + *neighbor_val;
                heap.push(Reverse((*neighbor_dist, neighbor_pos)));
            }
        }
    }

    println!("{}", distances.get(&[w-1, h-1]).unwrap());
}
