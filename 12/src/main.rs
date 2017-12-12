#![feature(test)]

extern crate petgraph;
extern crate test;
use petgraph::graphmap::UnGraphMap;
use petgraph::visit::{Bfs, Walker};
use std::fs::File;
use std::io::Read;

// You walk through the village and record the ID of each program and
// the IDs with which it can communicate directly (your puzzle input).
// Each program has one or more programs with which it can communicate,
// and these pipes are bidirectional; if 8 says it can communicate with 11,
// then 11 will say it can communicate with 8.

// You need to figure out how many programs are in the group that contains program ID 0.

// For example,
// suppose you go door-to-door like a travelling salesman and record the following list:

// 0 <-> 2
// 1 <-> 1
// 2 <-> 0, 3, 4
// 3 <-> 2, 4
// 4 <-> 2, 3, 6
// 5 <-> 6
// 6 <-> 4, 5

// In this example, the following programs are in the group that contains program ID 0:

//     Program 0 by definition.
//     Program 2, directly connected to program 0.
//     Program 3 via program 2.
//     Program 4 via program 2.
//     Program 5 via programs 6, then 4, then 2.
//     Program 6 via programs 4, then 2.

// Therefore, a total of 6 programs are in this group; all but program 1,
// which has a pipe that connects it to itself.

// How many programs are in the group that contains program ID 0?

// Your puzzle answer was 113.

fn node_count(contents: &String, node: u32) -> usize {
    let mut g = UnGraphMap::new();

    let f_contents = contents
        .chars()
        .filter(|&d| d != ',' && d != '<' && d != '>' && d != '-')
        .collect::<String>();

    for line in f_contents.lines() {
        let edges = line.split_whitespace()
            .map(|d| d.parse().unwrap())
            .collect::<Vec<u32>>();

        let mut edges_iter = edges.iter();

        let node = *edges_iter.next().unwrap();

        for neighbor in edges_iter {
            g.add_edge(node, *neighbor, 1);
        }
    }
    Bfs::new(&g, node).iter(&g).count()
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let count_0 = node_count(&contents, 0);
    println!("node count: {}", count_0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_node_count() {
        let contents = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"
            .to_string();
        assert_eq!(6, node_count(&contents, 0));
    }
    #[bench]
    fn bench_1(b: &mut Bencher) {
        let mut file = File::open("input.txt").expect("Unable to open");
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        b.iter(|| node_count(&contents, 0));
    }
}
