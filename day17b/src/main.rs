use std::io::{self, BufRead};

fn parse_range(s: &String) -> std::ops::RangeInclusive<i32>{
    let parts: Vec<&str> = s.split("..").collect();

    parts[0].parse::<i32>().unwrap()..=parts[1].parse::<i32>().unwrap()
}

fn step(pos: [i32; 2], vel: [i32; 2]) -> ([i32; 2], [i32; 2]) {
    let mut newpos = pos;
    let mut newvel = vel;

    newpos[0] += newvel[0];
    newpos[1] += newvel[1];

    newvel[0] -= newvel[0].signum();
    newvel[1] -= 1;

    (newpos, newvel)
}

fn main() {
    let stdin = io::stdin();

    let mut line = stdin.lock().lines().next().unwrap().unwrap();

    assert!(line.starts_with("target area: "));

    line = line.chars().skip(13).collect();
    let ranges: Vec<&str> = line.split(", ").collect();

    println!("{:?}", ranges);
    let xrange = parse_range(&ranges[0].chars().skip(2).collect());
    let yrange = parse_range(&ranges[1].chars().skip(2).collect());
    println!("x={:?} y={:?}", xrange, yrange);

    let mut pos: [i32; 2];
    let mut vel: [i32; 2];
    let mut maxy = i32::MIN;
    let mut count = 0;

    // determine the probe range
    let mut xprobe = [1i32 * xrange.start().signum(), if xrange.start().abs() > xrange.end().abs() { *xrange.start() } else { *xrange.end() }];
    xprobe.sort();
    println!("xprobe: {:?}", xprobe);

    for ivx in xprobe[0]..=xprobe[1] {
        // TODO: -300 is a guesstimate, needs a proper bound estimate.
        let mut yprobe = [*[*yrange.start(), 1].iter().min().unwrap(), -300*yrange.start().signum()];
        yprobe.sort();

        let mut localmaxy = 0;  // start poition
        for ivy in yprobe[0]..yprobe[1] {
            println!("yprobe: {:?}", yprobe);
            pos = [0i32; 2];
            vel = [ivx, ivy];
            println!("pos={:?}, vel={:?}", pos, vel);

            loop {
                if xrange.contains(&pos[0]) && yrange.contains(&pos[1]) {
                    // we're okay
                    println!("hit with vector: {},{}", ivx, ivy);
                    println!("hit:iv={:?} pos={:?}, vel={:?}", [ivx, ivy], pos, vel);
                    count += 1;
                    maxy = *[localmaxy, maxy].iter().max().unwrap();
                    break;
                }
                if vel[1].signum() == -1 && pos[1] < *yrange.start() {
                    println!("- outside range y");
                    break
                };

                if vel[0] == 0 && !xrange.contains(&pos[0]) {
                    println!("- outside range x");
                    break
                }

                //step
                let r = step(pos, vel);
                pos = r.0;
                vel = r.1;
                localmaxy = *[localmaxy, pos[1]].iter().max().unwrap();
                println!("- step pos={:?}, vel={:?}", pos, vel);
            }
        }
    }

    println!("{}", count);
}
