use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let lines = &mut stdin.lock().lines().map(|x| x.unwrap());

    let mut rules: HashMap<[char; 2], char> = HashMap::new();
    let mut template_pairs: HashMap<[char; 2], u64> = HashMap::new();
    let mut letter_count: HashMap<char, u64> = HashMap::new();

    let template: Vec<char> = lines.next().unwrap().chars().collect();

    for pair in template.windows(2) {
        *template_pairs.entry([pair[0], pair[1]]).or_insert(0) += 1;
    }

    for c in template {
        *letter_count.entry(c).or_default() += 1;
    }

    assert!(lines.next().unwrap() == "");

    for line in lines {
        let comps = line.split(" -> ").collect::<Vec<&str>>();
        let key: [char; 2] = <[char; 2]>::try_from(comps[0].chars().collect::<Vec<char>>()).unwrap();
        let value = comps[1].chars().next().unwrap();
        rules.insert(key, value);
    }

    let mut old: HashMap<[char; 2], u64>;
    let mut new = template_pairs;

    for _ in 0..40 {
        old = new.clone();
        new = HashMap::new();

        for (key, value) in old.iter() {
            let rule = rules.get(key);

            if let Some(&x) = rule {
                let pair_1 = [key[0], x];
                let pair_2 = [x, key[1]];

                *new.entry(pair_1).or_insert(0) += value;
                *new.entry(pair_2).or_insert(0) += value;
                *letter_count.entry(x).or_insert(0) += value;
            } else {
                *new.entry(*key).or_insert(0) += value;
            };
        }
    }

    println!("{}", letter_count.values().max().unwrap() - letter_count.values().min().unwrap());
}
