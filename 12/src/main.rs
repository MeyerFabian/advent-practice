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

//Pretty straight forward with petgraph, return the petgraph
fn create_ungraphmap(contents: &String, g: &mut UnGraphMap<u32, u32>) {
    let f_contents = contents
        .chars()
        .filter(|&d| d != ',' && d != '<' && d != '>' && d != '-')
        .collect::<String>();

    for line in f_contents.lines() {
        let mut edges = line.split_whitespace()
            .map(|d| d.parse().unwrap())
            .collect::<Vec<u32>>();

        let mut edges_iter = edges.iter();
        // First element in edges is the node
        // All following elements are edges from that node
        let node: u32 = *edges_iter.next().unwrap();
        for neighbor in edges_iter {
            g.add_edge(node, *neighbor, 1);
        }
        // If the node doesnt have any edges we still create it
        if edges.len() == 1 {
            g.add_node(node);
        }
    }
}

// Bfs makes our life easy we just do a Breadth First Search which
// will go through all elements somehow connected to our node
// and count how many that were
fn node_count(g: &UnGraphMap<u32, u32>, node: u32) -> usize {
    Bfs::new(&g, node).iter(&g).count()
}

//Instead of counting we return the elements which bfs has gone through.
fn nodes_connected(g: &UnGraphMap<u32, u32>, node: u32) -> Vec<u32> {
    Bfs::new(&g, node).iter(&g).collect::<Vec<u32>>()
}

// There are more programs than just the ones in the group containing program ID 0.
// The rest of them have no way of reaching that group,
// and still might have no way of reaching each other.

// A group is a collection of programs that can all communicate via pipes
// either directly or indirectly. The programs you identified just a moment ago
// are all part of the same group.
// Now, they would like you to determine the total number of groups.

// In the example above, there were 2 groups: one consisting of programs 0,2,3,4,5,6,
// and the other consisting solely of program 1.

// How many groups are there in total?

// Since we have to pay attention to borrows we do an infinite loop.
fn group_count(g: &UnGraphMap<u32, u32>) -> usize {
    let mut nodes = g.nodes().collect::<Vec<u32>>();
    let mut group_count = 0;
    while let Some(node) =nodes.pop(){
        group_count += 1;

        // We take the first node out of the graph and do a bfs for all its connected nodes.
        let group = Vec::new();
        group = nodes_connected(&g, node);

        // Then we filter out all nodes that were in that group from all left graph nodes.
        // Were left with all nodes that havent been traversed ones with bfs.
        let filtered_notes = nodes
            .iter()
            .filter(|&d| !group.contains(d))
            .map(|d| *d)
            .collect::<Vec<u32>>();
        nodes = filtered_notes;

    }
    group_count
}
fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let mut g = UnGraphMap::new();
    let _ = create_ungraphmap(&contents, &mut g);
    let count_0 = node_count(&g, 0);
    println!("node count: {}", count_0);

    //PART 2
    let num_groups = group_count(&g);
    println!("group count {}", num_groups);
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
        let mut g = UnGraphMap::new();
        let _ = create_ungraphmap(&contents, &mut g);
        assert_eq!(6, node_count(&g, 0));
    }
    #[test]
    fn test_group_count() {
        let contents = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"
            .to_string();
        let mut g = UnGraphMap::new();
        let _ = create_ungraphmap(&contents, &mut g);
        assert_eq!(2, group_count(&g));
    }
    #[bench]
    fn bench_1(b: &mut Bencher) {
        let mut file = File::open("input.txt").expect("Unable to open");
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        let mut g = UnGraphMap::new();
        let _ = create_ungraphmap(&contents, &mut g);
        b.iter(|| node_count(&g, 0));
    }
}
