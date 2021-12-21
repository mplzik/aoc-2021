use std::io::{self, BufRead};

fn sample(map: &Vec<Vec<char>>, x: isize, y: isize, outside: char) -> char {
    if y < 2 || x < 2 || x > (map.len() + 1) as isize || y > (map.len() + 1) as isize {
        return outside;
    }

    map[(y - 2) as usize][(x - 2) as usize]
}

fn enhance(map: &Vec<Vec<char>>, p: &Vec<char>, outside: char) -> (Vec<Vec<char>>, char) {
    let mut ret: Vec<Vec<char>> = vec![];

    for y in 0isize..(map.len() + 4) as isize {
        ret.push(vec![]);
        for x in 0isize..(map[0].len() + 4) as isize {
            // sample

            let mut idx = 0;
            for neighbor in [
                [-1, -1],
                [-1, 0],
                [-1, 1],
                [0, -1],
                [0, 0],
                [0, 1],
                [1, -1],
                [1, 0],
                [1, 1],
            ] {
                idx *= 2;
                if sample(map, x + neighbor[1], y + neighbor[0], outside) == '#' {
                    idx += 1;
                }
            }
            ret[y as usize].push(p[idx]);
        }
    }

    (
        ret,
        match outside {
            '.' => p[0b000000000],
            '#' => p[0b111111111],
            _ => panic!("Unknown outside char!"),
        },
    )
}

fn dump(map: &Vec<Vec<char>>) {
    for line in map {
        println!("{}", line.iter().collect::<String>());
    }
}

fn main() {
    let stdin = io::stdin();

    let program = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    assert_eq!(program.len(), 512);
    let mut map: Vec<Vec<char>> = vec![];
    let mut outside = '.';

    stdin.lock().lines().next().unwrap().unwrap(); // whitespace

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        map.push(line.chars().collect())
    }

    dump(&map);

    for _ in 0..2 {
        let r = enhance(&map, &program, outside);
        map = r.0;
        outside = r.1;
    }

    let mut ones = 0;
    for line in map.iter() {
        ones += line.iter().filter(|&&x| x == '#').count();
    }

    dump(&map);
    println!("{}", ones);
}
