use std::collections::HashMap;
use std::io::{self, BufRead};

fn small(s: &str) -> bool {
    s.chars().map(|x| ('a'..='z').contains(&x)).all(|x| x)
}

fn trace<'a>(
    graph: &'a HashMap<String, Vec<String>>,
    visited: &mut HashMap<&'a str, u32>,
    pos: &'a str,
) -> u32 {
    let mut ret = 0;

    if pos == "end" {
        // `end` won't be ever visited twice
        return 1;
    }

    if small(&pos) {
        let cpos = *visited.entry(pos).or_insert(0);
        if pos == "start" && cpos > 0 {
            // We can't visit start twice
            return 0;
        };
        if cpos == 1 && visited.values().any(|&x| x > 1) {
            // at most one vertex can be visited twice
            return 0;
        }
        *visited.entry(pos).or_insert(0) += 1;
    };

    for neighbor in graph[pos].iter() {
        if *visited.get(neighbor as &str).unwrap_or(&0) < 2 {
            ret += trace(graph, visited, neighbor);
        }
    }

    if small(&pos) {
        *visited.entry(pos).or_insert(0) -= 1;
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
            .or_insert(vec![])
            .push(comps[1].to_string());
        graph
            .entry(comps[1].to_string())
            .or_insert(vec![])
            .push(comps[0].to_string());
    }

    let count = trace(&graph, &mut HashMap::new(), "start");

    println!("{}", count);
}
