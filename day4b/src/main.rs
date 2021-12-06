use std::io::{self, BufRead};

type Bingo = [[u8; 5]; 5];

fn read_bingo(lines: &mut std::io::Lines<std::io::StdinLock>) -> Option<Bingo> {
    let mut ret: Bingo = [[0; 5]; 5];

    for l in 0..5 {
        let line = lines.next()?;
        let nums: Vec<u8> = line
            .unwrap()
            .split(' ')
            .filter(|x| *x != "")
            .map(|x| x.parse().unwrap())
            .collect();

        ret[l] = <[u8; 5]>::try_from(nums).unwrap();
    }

    Some(ret)
}

fn check_bingo(board: &Bingo, draw: &Vec<u8>) -> Option<(u8, u32)> {
    let mut b: Bingo = *board;
    let mut rowhits = [0; 5];
    let mut colhits = [0; 5];

    for (i, d) in draw.iter().enumerate() {
        for y in 0..5 {
            for x in 0..5 {
                if b[y][x] != *d {
                    continue;
                }
                b[y][x] = u8::MAX;
                rowhits[y] += 1;
                colhits[x] += 1;

                if rowhits[y] == 5 || colhits[x] == 5 {
                    let ret = Some((
                        i as u8,
                        b.iter()
                            .flatten()
                            .filter(|x| **x != u8::MAX)
                            .map(|x| (*x) as u32)
                            .sum::<u32>()
                            * (*d as u32),
                    ));
                    return ret;
                }
            }
        }
    }

    None
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut boards: Vec<Bingo> = Vec::new();

    let draw: Vec<u8> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    loop {
        lines.next(); // eat an empty line
        let out = read_bingo(&mut lines);
        match out {
            None => break,
            Some(b) => boards.push(b),
        }
    }

    let winner = boards
        .iter()
        .map(|b| check_bingo(b, &draw).unwrap())
        .max_by_key(|x| x.0)
        .unwrap();
    println!("{}", winner.1);
}
