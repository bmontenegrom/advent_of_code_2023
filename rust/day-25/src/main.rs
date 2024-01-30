use std::collections::HashMap;

use rustworkx_core::{connectivity::stoer_wagner_min_cut, petgraph::graph::UnGraph};

//solve with minimum cut
fn solve(input: &str) -> usize {
    let mut graph: UnGraph<&str, ()> = rustworkx_core::petgraph::Graph::new_undirected();
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(": ");
        let node = parts.next().unwrap();
        let node = *nodes.entry(node).or_insert_with(|| graph.add_node(node));
        let edges = parts.next().unwrap().split(' ');
        for edge in edges {
            let edge = *nodes.entry(edge).or_insert_with(|| graph.add_node(edge));
            graph.add_edge(node, edge, ());
        }
    }

    match stoer_wagner_min_cut(&graph, |_| Ok::<i32, ()>(1)) {
        Err(_) => panic!("shouldn't happen"),
        Ok(None) => panic!("shouldn't happen"),
        Ok(Some((_, partition))) =>  partition.len() * (nodes.len() - partition.len()),
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let now = std::time::Instant::now();
    println!("Part 1:{} in  {:?}", solve(input), now.elapsed());
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("input_test.txt");
        assert_eq!(solve(input), 54);
    }
}