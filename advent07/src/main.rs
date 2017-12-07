use std::fs::File;
use std::io::Read;
// One program at the bottom supports the entire tower.
// It's holding a large disc, and on the disc are balanced several more sub-towers.
// At the bottom of these sub-towers, standing on the bottom disc, are other programs,
// each holding their own disc, and so on. At the very tops of these sub-sub-sub-...-towers,
// many programs stand simply keeping the disc below them balanced but with no disc of their own.

// You offer to help, but first you need to understand the structure of these towers.
// You ask each program to yell out their name, their weight, and (if they're holding a disc)
// the names of the programs immediately above them balancing on that disc.
// You write this information down (your puzzle input). Unfortunately, in their panic,
// they don't do this in an orderly fashion; by the time you're done,
// you're not sure which program gave which information.

// For example, if your list is the following:

// pbga (66)
// xhth (57)
// ebii (61)
// havc (66)
// ktlj (57)
// fwft (72) -> ktlj, cntj, xhth
// qoyq (66)
// padx (45) -> pbga, havc, qoyq
// tknk (41) -> ugml, padx, fwft
// jptl (61)
// ugml (68) -> gyxo, ebii, jptl
// gyxo (61)
// cntj (57)

// ...then you would be able to recreate the structure of the towers that looks like this:

//                 gyxo
//               /
//          ugml - ebii
//        /      \
//       |         jptl
//       |
//       |         pbga
//      /        /
// tknk --- padx - havc
//      \        \
//       |         qoyq
//       |
//       |         ktlj
//        \      /
//          fwft - cntj
//               \
//                 xhth

// In this example, tknk is at the bottom of the tower (the bottom program),
// and is holding up ugml, padx, and fwft.
// Those programs are, in turn, holding up other programs; in this example,
// none of those programs are holding up any other programs,
// and are all the tops of their own towers.
// (The actual tower balancing in front of you is much larger.)

// Before you're ready to help them, you need to make sure your information is correct.
// What is the name of the bottom program?

fn find_root_node<'a>(names: &[&'a str], flat_children: &[&&str]) -> Option<&'a str> {
    for node in names {
        let found = flat_children.iter().find(|&&children| node == children);
        if found == None {
            return Some(node);
        }
    }
    None
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let filter = contents
        .chars()
        .filter(|&d| d != '(' && d != ')' && d != ',')
        .collect::<String>();

    let mut names = Vec::new();

    let mut weights = Vec::new();
    let children: Vec<Vec<&str>> = filter
        .lines()
        .map(|line| {
            let split_line = line.split_whitespace()
                .filter(|&d| d != "->")
                .collect::<Vec<&str>>();
            names.push(split_line[0]);
            weights.push(split_line[1]);
            split_line.iter().skip(2).map(|x| *x).collect::<Vec<&str>>()
        })
        .collect();

    let flat_children: Vec<&&str> = children.iter().flat_map(|x| x).collect();
    let root_node = find_root_node(&names, &flat_children);
    match root_node {
        Some(root) => println!("{}", root),
        None => println!("No root node identified!"),
    };
}
