use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum SFNum {
    Literal(u32),
    Ref(usize),
}

#[derive(Debug, Clone, Copy)]
struct Node {
    parent: usize,
    children: [SFNum; 2],
}

use SFNum::*;
fn parse_num<'a>(tree: &mut Vec<Node>, parent: usize, s: &'a [char]) -> (&'a [char], SFNum) {
    let mut pos = s;

    if pos[0] == '[' {
        tree.push(Node {
            parent: parent,
            children: [Literal(u32::MAX), Literal(u32::MAX)],
        });
        let node_pos = tree.len() - 1;

        // we're starting a pair here
        pos = &pos[1..];
        let (mut pos, num1) = parse_num(tree, node_pos, pos);

        assert_eq!(pos[0], ',');
        pos = &pos[1..];

        let (mut pos, num2) = parse_num(tree, node_pos, pos);

        assert_eq!(pos[0], ']');
        pos = &pos[1..];

        let node = tree.get_mut(node_pos).unwrap();
        node.children = [num1, num2];

        return (pos, Ref(node_pos));
    } else {
        // number
        let mut digits: String = String::new();

        while ('0'..='9').contains(&pos[0]) {
            digits.push(pos[0]);
            pos = &pos[1..]
        }
        let literal = digits.parse::<u32>().unwrap();
        return (pos, Literal(literal));
    }
}

fn nextnum(trees: &Vec<Node>, origin: usize, lr: usize) -> (usize, usize) {
    // return value: (position, left = 0/right = 1)
    let mut prevpos = origin;
    let mut cpos = trees[origin].parent;

    // Descend down
    while cpos != usize::MAX {
        let node = trees.get(cpos).unwrap();
        // If we're coming from the right subtree, the answer is to the left
        if let Ref(right) = node.children[1 - lr] {
            if right == prevpos {
                match node.children[lr] {
                    Literal(_) => return (cpos, lr),
                    Ref(x) => {
                        cpos = x;
                        // traverse the leftmost items until a literal is found
                        break;
                    }
                }
            }
        }
        // If we're coming from the left subtree, just continue in descent
        prevpos = cpos;
        cpos = node.parent;
    }

    if cpos == usize::MAX {
        return (cpos, 2); // this was the first/last value
    }

    // Bubble up
    loop {
        let node = trees.get(cpos).unwrap();

        match node.children[1 - lr] {
            Literal(_) => return (cpos, 1 - lr),
            Ref(child) => cpos = child,
        }
    }
}

fn explode(trees: &mut Vec<Node>, level: usize, pos: usize) -> bool {
    let node = trees.get_mut(pos).unwrap().clone();

    if level < 4 {
        for child in node.children {
            if let Ref(childpos) = child {
                if explode(trees, level + 1, childpos) {
                    return true;
                };
            };
        }
        return false;
    }

    if level == 4 {
        // explode
        // distribute the values to the neighbors
        for (child, (adjpos, adjidx)) in [
            (node.children[0], nextnum(trees, pos, 0)), // left value and neighbor
            (node.children[1], nextnum(trees, pos, 1)), // right value and neighbor
        ] {
            if adjpos == usize::MAX {
                continue; // there's no neighbor in this direction
            }
            if let Literal(neigh) = trees[adjpos].children[adjidx] {
                if let Literal(nodeval) = child {
                    trees[adjpos].children[adjidx] = Literal(neigh + nodeval);
                }
            }
        }

        // set myself as zero
        let mut parent = trees.get_mut(node.parent).unwrap();
        for i in 0..parent.children.len() {
            if let Ref(parentref) = parent.children[i] {
                if parentref == pos {
                    parent.children[i] = Literal(0);
                    return true;
                }
            }
        }
        panic!("Couldn't find parent for a child while expliding");
    }

    false
}

fn do_split(trees: &mut Vec<Node>, pos: usize, lr: usize) {
    let node = trees[pos];

    if let Literal(val) = node.children[lr] {
        assert!(val >= 10);

        trees.push(Node {
            parent: pos,
            children: [Literal(val / 2), Literal(val - val / 2)],
        });
        let split_pos = trees.len() - 1;

        trees.get_mut(pos).unwrap().children[lr] = Ref(split_pos);
    } else {
        panic!("Trying to perform split on a non-value field!");
    }
}

fn split(trees: &mut Vec<Node>, pos: usize) -> bool {
    let node = trees.get(pos).unwrap().clone();

    for i in 0..node.children.len() {
        let ret = match node.children[i] {
            Ref(x) => split(trees, x),
            Literal(x) => {
                if x >= 10 {
                    do_split(trees, pos, i);
                    true
                } else {
                    false
                }
            }
        };

        if ret == true {
            return ret;
        };
    }

    return false;
}

fn sum_sfn(trees: &mut Vec<Node>, a: usize, b: usize) -> SFNum {
    trees.push(Node {
        parent: usize::MAX,
        children: [Ref(a), Ref(b)],
    });

    let sum_node = trees.len() - 1;
    trees[a].parent = sum_node;
    trees[b].parent = sum_node;

    loop {
        if explode(trees, 0, sum_node) {
            continue;
        }

        if split(trees, sum_node) {
            continue;
        };
        break;
    }

    return Ref(sum_node);
}

fn dump(trees: &Vec<Node>, root: usize) -> String {
    let values: String = trees[root]
        .children
        .iter()
        .map(|child| match child {
            Literal(x) => x.to_string(),
            Ref(x) => dump(trees, *x),
        })
        .collect::<Vec<String>>()
        .join(",");

    String::from("[") + &values + "]" // + "@" + &root.to_string()
}

fn as_ref(n: &SFNum) -> usize {
    if let Ref(x) = n {
        return *x;
    }

    panic!("Couldn't unwrap {:?} as Ref", n);
}

fn magnitude(tree: &Vec<Node>, pos: usize) -> u32 {
    let node = tree[pos];

    let left = match node.children[0] {
        Literal(x) => x,
        Ref(x) => magnitude(tree, x),
    };

    let right = match node.children[1] {
        Literal(x) => x,
        Ref(x) => magnitude(tree, x),
    };

    3 * left + 2 * right
}

fn main() {
    let stdin = io::stdin();
    let mut trees: Vec<Node> = vec![];
    let mut previous = usize::MAX;
    let mut sum: SFNum = Literal(u32::MAX);

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let chars = line.chars().collect::<Vec<char>>();
        let (remaining, num) = parse_num(&mut trees, usize::MAX, &chars);

        assert_eq!(remaining, &[]);

        if let Ref(current) = num {
            if previous == usize::MAX {
                previous = current
            } else {
                sum = sum_sfn(&mut trees, previous, current);
                if let Ref(r) = sum {
                    previous = r;
                } else {
                    panic!("This shouldn't happen!");
                }
            }
        };

        println!("{}", dump(&trees, as_ref(&num)));
    }

    println!("{}", dump(&trees, as_ref(&sum)));
    println!("magnitude: {}", magnitude(&trees, as_ref(&sum)));
}
