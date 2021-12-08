use std::io::{self, BufRead};

type Digit = [u32; 7];

const D: [Digit; 10] = [
    [1, 1, 1, 0, 1, 1, 1],
    [0, 0, 1, 0, 0, 1, 0],
    [1, 0, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 0, 1, 1],
    [0, 1, 1, 1, 0, 1, 0],
    [1, 1, 0, 1, 0, 1, 1],
    [1, 1, 0, 1, 1, 1, 1],
    [1, 0, 1, 0, 0, 1, 0],
    [1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 0, 1, 1],
];

fn let2vec(letters: &str) -> Digit {
    let mut ret = [0; 7];

    for l in letters.bytes() {
        ret[(l - ('a' as u8)) as usize] = 1;
    }
    ret
}

fn main() {
    let stdin = io::stdin();
    let mut s = 0;

    let right_sizes: Vec<usize> = [D[1], D[4], D[7], D[8]].iter().map(|x| x.iter().cloned().sum::<u32>() as usize).collect();
    println!("right_sizes = {:?}", right_sizes);

    for line in stdin.lock().lines().map(Result::unwrap) {
        let sig_out: Vec<&str> = line.split(" | ").collect();
        //let signals: Vec<Digit> = sig_out[0].split(' ').map(let2vec).collect();
        let outputs: Vec<&str> = sig_out[1].split(' ').collect();

        let matching: Vec<&str> = outputs.iter().cloned()
            .filter(|x| right_sizes.contains(&x.len())).collect();

        println!("matching: {:?}", matching);

        s += matching.len();
    }

    println!("{}", s);
}
