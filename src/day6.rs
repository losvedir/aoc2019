use petgraph::Graph;
// use petgraph::NodeIndex;
use std::collections::HashMap;
use std::fs;

pub fn puzzle1() {
    let ct = count_orbits(get_input());
    println!("day6 puzzle1: {}", ct);
}

fn get_input() -> String {
    fs::read_to_string("src/day6input.txt").unwrap()
}

fn count_orbits(map: String) -> i32 {
    let mut nodes = HashMap::new();
    let mut g: Graph<(), i32> = Graph::new();
    for line in map.split_whitespace() {
        let mut ns = line.split(")");
        let n1 = ns.next().unwrap();
        let n2 = ns.next().unwrap();

        let ix1 = if nodes.contains_key(n1) {
            *nodes.get(n1).unwrap()
        } else {
            let ix = g.add_node(());
            nodes.insert(n1, ix);
            ix
        };

        let ix2 = if nodes.contains_key(n2) {
            *nodes.get(n2).unwrap()
        } else {
            let ix = g.add_node(());
            nodes.insert(n2, ix);
            ix
        };

        g.extend_with_edges(&[(ix1, ix2), (ix2, ix1)]);
    }

    let you = *nodes.get("YOU").unwrap();
    let san = *nodes.get("SAN").unwrap();

    let (cost, _) = petgraph::algo::astar(&g, you, |n| n == san, |_e| 1, |_| 0).unwrap();
    println!("puzzle2: {}", cost - 2);

    let com = *nodes.get("COM").unwrap();
    petgraph::algo::dijkstra(&g, com, None, |_| 1)
        .iter()
        .map(|(_k, cost)| cost)
        .sum()
    // let (wts, _) = petgraph::algo::bellman_ford(g, com).unwrap();
    // wts.sum()
}
