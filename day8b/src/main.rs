use std::io::{self, BufRead};

type Digit = u32;

fn let2dig(letters: &str) -> Digit {
    let mut ret: Digit = 0;

    for l in letters.bytes() {
        ret |= 1<< 6 -(l - ('a' as u8)) as usize;
    }
    ret
}

fn justone(d: &Vec<Digit>) -> Digit {
    if d.len() != 1 {
        panic!("Unexpected amount of cases found as a result: {:?}", d);
    }

    return d[0];
}

fn idx(v: &[u32; 10], d: Digit) -> u32 {
    match v.iter().position(|&x| x == d) {
        Some(pos) => pos as u32,
        None => u32::MAX,
    }
}

fn main() {
    let stdin = io::stdin();
    let mut s: u32 = 0;

    for line in stdin.lock().lines().map(Result::unwrap) {
        let sig_out: Vec<&str> = line.split(" | ").collect();
        let signals: Vec<Digit> = sig_out[0].split(' ').map(let2dig).collect();
        let outputs: Vec<Digit> = sig_out[1].split(' ').map(let2dig).collect();
        let mut ds: [Digit; 10] = [0; 10];
        let mut by_size:[Vec<Digit>; 8] = [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

        for i in 0..signals.len() {
            by_size[signals[i].count_ones() as usize].push(signals[i]);
        }

        ds[1] = justone(&by_size[2]);
        ds[4] = justone(&by_size[4]);
        ds[7] = justone(&by_size[3]);
        ds[8] = justone(&by_size[7]);


        ds[3] = justone(&by_size[5].iter().cloned().filter(|x| (x & ds[1]).count_ones() == 2).collect());
        
        // Elements with 6 ones.

        // There are three 6-sized elements; out of these, only 9 is a superset of 3 (and the 9A3 yields 5 ones)
        ds[9] = justone(&by_size[6].iter().cloned().filter(|x| (x & ds[3]).count_ones() == 5).collect());

        // 6-sized remaining: 0, 6. 0 is a superset of 1
        ds[0] = justone(&by_size[6].iter().cloned().filter(|&x| (x & ds[1]) == ds[1] && x != ds[9]).collect());

        // Remaining 6-sized element is 6
        ds[6] = justone(&by_size[6].iter().cloned().filter(|&x| x != ds[0] && x != ds[9]).collect());

        // Elements with 5 ones remaining :5, 2. Only 5 has 5 bits common with 6.
        ds[5] = justone(&by_size[5].iter().cloned().filter(|&x| (x & ds[6]).count_ones() == 5).collect());

        // Elements with 5 ones; remaiing 2
        ds[2] = justone(&by_size[5].iter().cloned().filter(|x| ![ds[5], ds[3]].contains(x)).collect());

        // Reconstruct the letters

        let sum = 1000*idx(&ds, outputs[0]) + 100*idx(&ds, outputs[1]) + 10*idx(&ds, outputs[2]) + 1*idx(&ds, outputs[3]);
        s += sum
    }

    println!("{}", s);
}
