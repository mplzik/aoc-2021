use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn small(s: &str) -> bool {
    s.chars()
        .map(|x| ('a'..='z').contains(&x))
        .all(|x| x == true)
}

fn trace<'a>(
    graph: &'a HashMap<String, Vec<String>>,
    visited: &mut HashSet<&'a str>,
    pos: &'a str,
) -> u32 {
    let mut ret = 0;

    if pos == "end" {
        return 1;
    }

    if small(&pos) {
        visited.insert(pos);
    };

    for neighbor in graph[pos].iter() {
        if !visited.contains(neighbor as &str) {
            ret += trace(graph, visited, neighbor);
        }
    }

    if small(&pos) {
        visited.remove(pos);
    };

    ret
}

fn main() {
    let stdin = io::stdin();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let comps: Vec<&str> = line.split('-').collect();
        assert!(comps.len() == 2);
        graph
            .entry(comps[0].to_string())
            .or_insert(Vec::new())
            .push(comps[1].to_string());
        graph
            .entry(comps[1].to_string())
            .or_insert(Vec::new())
            .push(comps[0].to_string());
    }

    let count = trace(&graph, &mut HashSet::new(), "start");

    println!("{}", count);
}
