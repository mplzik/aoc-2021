use std::io::{self, BufRead};
use std::u8;

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u32,
    type_id: u32,

    payload: PacketPayload,
}

#[derive(Debug, PartialEq, Eq)]
enum PacketPayload {
    Literal(u128),
    Operator(Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq)]
enum SizeLimit {
    BitsSize(u32),
    SubpacketsCount(u32),
}

use PacketPayload::*;
use SizeLimit::*;

fn str2bits(s: &str) -> Vec<u8> {
    s.chars()
        .map(|x| {
            let n = u8::from_str_radix(&x.to_string(), 16).unwrap();
            [(n & 0x8) >> 3, (n & 0x4) >> 2, (n & 0x2) >> 1, n & 0x1]
        })
        .flatten()
        .collect()
}

fn bits2int(i: &[u8]) -> u128 {
    let mut ret = 0u128;

    for i in i.iter() {
        ret *= 2;
        ret += *i as u128;
    }
    ret
}

fn parse_bits(data: &[u8], limit: SizeLimit) -> (&[u8], Vec<Packet>) {
    let mut ret: Vec<Packet> = vec![];
    let mut d = data;

    while d.len() > 0 {
        match limit {
            BitsSize(x) => {
                if (data.len() - d.len()) >= x as usize {
                    break;
                }
            }
            SubpacketsCount(x) => {
                if ret.len() == x as usize {
                    break;
                }
            }
        }
        let version = bits2int(&d[0..3]);
        d = &d[3..];
        let type_id = bits2int(&d[0..3]);
        d = &d[3..];

        if type_id == 4 {
            // parse literal
            let mut literal: Vec<u8> = vec![];
            loop {
                let bits = &d[..5];
                d = &d[5..];
                literal.extend(bits[1..].iter());

                if bits[0] == 0 {
                    break;
                }
            }
            let new_packet = Packet {
                version: version as u32,
                type_id: type_id as u32,
                payload: Literal(bits2int(literal.as_slice())),
            };
            ret.push(new_packet);
        } else {
            // parse operator
            let op_type = d[0];
            d = &d[1..];
            let limit_type = match op_type {
                0 => {
                    let bits = &d[..15];
                    d = &d[15..];
                    SizeLimit::BitsSize(bits2int(bits) as u32)
                }
                1 => {
                    let bits = &d[..11];
                    d = &d[11..];
                    SizeLimit::SubpacketsCount(bits2int(bits) as u32)
                }
                _ => panic!("Unexpected value for limit_type"),
            };

            let newpkts = parse_bits(d, limit_type);

            d = newpkts.0;
            let new_packet = Packet {
                version: version as u32,
                type_id: type_id as u32,
                payload: PacketPayload::Operator(newpkts.1),
            };
            ret.push(new_packet);
        }
    }

    (d, ret)
}

fn eval(p: &Packet) -> u128 {
    if let Literal(v) = p.payload {
        return v;
    };

    // evaluate children
    if let Operator(o) = &p.payload {
        let results: Vec<u128> = o.iter().map(eval).collect();

        match p.type_id {
            0 => results.iter().sum(),
            1 => results.iter().product(),
            2 => *results.iter().min().unwrap(),
            3 => *results.iter().max().unwrap(),
            5 => (results[0] > results[1]) as u128,
            6 => (results[0] < results[1]) as u128,
            7 => (results[0] == results[1]) as u128,
            x => {
                panic!("Unknown type id {}", x);
            }
        }
    } else {
        panic!("This shouldn't happen");
    }
}

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let bits = str2bits(&line);
        let parsed = parse_bits(bits.as_slice(), SizeLimit::SubpacketsCount((1) as u32));

        println!("{}", eval(&(parsed.1[0])));
    }
}
