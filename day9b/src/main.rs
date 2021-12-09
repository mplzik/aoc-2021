use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

type Point = [usize; 2];

const NEIGHBORS: [[isize; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

fn neighbors(map: &Vec<Vec<u8>>, p: Point) -> Vec<Point> {
    let mapw = map[0].len() as isize;
    let maph = map.len() as isize;
    NEIGHBORS
        .iter()
        .map(|[x, y]| [x + p[0] as isize, y + p[1] as isize]) // absolute position
        .filter(|[x, y]| (0isize..mapw).contains(x) && (0isize..maph).contains(y)) // remove points out of bounds
        .map(|[x, y]| [x as usize, y as usize]) // convert to usize
        .collect()
}

fn basin(map: &Vec<Vec<u8>>, x: usize, y: usize) -> HashSet<Point> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();

    visited.insert([x, y]);
    queue.push_back([x, y]);

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        for n in neighbors(&map, current) {
            if map[n[1]][n[0]] != 9 {
                if !visited.contains(&n) {
                    visited.insert(n);
                    queue.push_back(n);
                }
            }
        }
    }

    visited
}

fn main() {
    let stdin = io::stdin();
    let mut map: Vec<Vec<u8>> = Vec::new();

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let heights: Vec<u8> = line
            .chars()
            .map(|x| (x.to_string().parse().unwrap()))
            .collect();
        map.push(heights)
    }

    let mut basins: Vec<HashSet<Point>> = Vec::new();
    for y in 0..map.len() {
        let line = &map[y];
        for x in 0..line.len() {
            let c = map[y][x] as u32;

            let is_low_point = neighbors(&map, [x, y])
                .iter()
                .map(|n| c < map[n[1]][n[0]] as u32)
                .all(|x| x == true);
            if is_low_point {
                basins.push(basin(&map, x, y));
            };
        }
    }

    basins.sort_by_key(HashSet::len);
    println!(
        "{:?}",
        &basins[basins.len() - 3..]
            .iter()
            .map(HashSet::len)
            .product::<usize>()
    );
}
