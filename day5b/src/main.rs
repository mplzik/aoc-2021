use std::io::{self, BufRead};
use std::collections::{HashMap};

type Point = [i32; 2];

fn main() {
    let stdin = io::stdin();
    let mut points = HashMap::<Point, u32>::new();

    for line in stdin.lock().lines().map(Result::unwrap) {
        let l: Vec<&str> = line.split(" -> ").collect();
        let p1: Point = Point::try_from(l[0].split(',').map(|x| x.parse().unwrap()).collect::<Vec<i32>>()).unwrap();
        let p2: Point = Point::try_from(l[1].split(',').map(|x| x.parse().unwrap()).collect::<Vec<i32>>()).unwrap();

        let dx = (p2[0] - p1[0]).signum();
        let dy = (p2[1] - p1[1]).signum();

        let mut pos = p1;

        while !(pos[0] == p2[0] && pos[1] == p2[1]) {
            *(points.entry(pos).or_default()) += 1;
            pos[0] += dx;
            pos[1] += dy;
        }
        *(points.entry(pos).or_default()) += 1;
    }

    println!("{}", points.iter().filter(|(_, &v)| v > 1).count());
}
