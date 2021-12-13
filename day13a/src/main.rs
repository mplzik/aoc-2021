type Point = [usize; 2];
use std::collections::HashSet;

fn fold(page: &HashSet<Point>, axis: &str, pos: usize) -> HashSet<Point> {
    let mut ret: HashSet<Point> = HashSet::new();

    for point in page.iter().cloned() {
        let newpoint = match axis {
            "x" if point[0] > pos => [(2 * pos as isize - point[0] as isize) as usize, point[1]],
            "y" if point[1] > pos => [point[0], (2 * pos as isize - point[1] as isize) as usize],
            _ => point,
        };
        ret.insert(newpoint);
    }
    ret
}

use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let lines = &mut stdin.lock().lines();
    let mut points: HashSet<Point> = HashSet::new();

    for line in lines.map(|x| x.unwrap()).take_while(|x| x != "") {
        let point = <Point>::try_from(
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        )
        .unwrap();
        points.insert(point);
    }

    let line = lines.map(|x| x.unwrap()).next().unwrap();
    assert!(line.starts_with("fold along "));

    let from = "fold along ".len();
    let fold_ins = &line[from..].split('=').collect::<Vec<&str>>();
    points = fold(&points, fold_ins[0], fold_ins[1].parse::<usize>().unwrap());

    println!("{}", points.len());
}
