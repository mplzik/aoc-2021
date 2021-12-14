use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = &mut stdin.lock().lines().map(|x| x.unwrap());
    let mut rules: HashMap<String, char> = HashMap::new();

    let template = lines.next().unwrap();

    assert!(lines.next().unwrap() == "");

    for line in lines {
        let comps = line.split(" -> ").collect::<Vec<&str>>();
        rules.insert(comps[0].to_string(), comps[1].chars().next().unwrap());
    }

    let mut old: String;
    let mut new = template;

    for _ in 0..10 {
        old = new.to_string();
        new = String::new();
        new.push_str(&old[0..1]);

        for pos in 0..old.len() - 1 {
            let from = &old[pos..pos + 2];
            let rule = rules.get(from);

            if let Some(x) = rule {
                new.push(*x);
                new.push_str(&from[1..]);
            } else {
                new.push_str(&from[1..]);
            };
        }
    }

    let mut freq: HashMap<char, u64> = HashMap::new();
    for c in new.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    println!(
        "{}",
        freq.values().max().unwrap() - freq.values().min().unwrap()
    );
}
