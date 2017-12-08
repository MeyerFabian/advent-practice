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
fn find_root_node<'a>(names: &[&'a str], flat_children: &[&&str]) -> Option<(usize, &'a str)> {
    for (i, node) in names.iter().enumerate() {
        let found = flat_children.iter().find(|&&children| node == children);
        if found == None {
            return Some((i, node));
        }
    }
    None
}


fn error_recovery(
    names: &[&str],
    weights: &mut [u32],
    sums: &mut [u32],
    try: usize,
    ind: usize,
    children: &[&str],
) {
    let filter: Vec<u32> = sums.iter()
        .filter(|&&x| x != sums[try])
        .map(|x| *x)
        .collect();
    if filter.len() == 1 {
        println!("Identified error in {}:", names[ind]);
        println!("childs:\t\t{:?}", sums);
        let (i, _) = sums.iter()
            .enumerate()
            .find(|&(_, &x)| x == filter[try])
            .unwrap();
        let error_name = children[i];
        println!("about name:\t{}", error_name);
        let (j, _) = names
            .iter()
            .enumerate()
            .find(|&(_, &d)| d == error_name)
            .unwrap();
        println!("with weight:\t{}", weights[j]);
        weights[j] += sums[try] - sums[i];
        sums[i] = sums[try];
        println!("Recovered from error!");
        println!("New weight:\t{}", weights[j]);
        println!("Childs now:\t{:?}", sums);
        println!();
    }
}

fn traverse(
    names: &[&str],
    mut weights: &mut [u32],
    children: &Vec<Vec<&str>>,
    ind: usize,
    childs_of_this: &mut Vec<u32>,
) -> u32 {
    let mut childs_of_all_childs: Vec<Vec<u32>> = vec![];
    let mut sums: Vec<u32> = vec![];
    for child in children[ind].iter() {
        let mut childs_of_this_child: Vec<u32> = vec![];
        let (i, _) = names.iter().enumerate().find(|&(_, d)| d == child).unwrap();
        let val = traverse(
            &names,
            &mut weights,
            &children,
            i,
            &mut childs_of_this_child,
        );
        childs_of_this.push(val);
        //let sum:u32 =  childs_of_this_child.iter().sum();
        sums.push(val);
        childs_of_all_childs.push(childs_of_this_child);
    }
    if sums.len() > 2 {
        error_recovery(&names, &mut weights, &mut sums, 0, ind, &children[ind]);
        error_recovery(&names, &mut weights, &mut sums, 1, ind, &children[ind]);
    }

    let mut sum: u32 = 0;
    if sums.len() > 0 {
        sum = sums.iter().sum();
    }
    sum + weights[ind]
}
fn traverse_tree(
    names: &[&str],
    mut weights: &mut [u32],
    children: &Vec<Vec<&str>>,
    root_ind: usize,
) {
    let mut child_weights = vec![];

    traverse(
        &names,
        &mut weights,
        &children,
        root_ind,
        &mut child_weights,
    );
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

    let mut weights: Vec<u32> = Vec::new();
    let children: Vec<Vec<&str>> = filter
        .lines()
        .map(|line| {
            let split_line = line.split_whitespace()
                .filter(|&d| d != "->")
                .collect::<Vec<&str>>();
            names.push(split_line[0]);
            weights.push(split_line[1].parse().unwrap());
            split_line.iter().skip(2).map(|x| *x).collect::<Vec<&str>>()
        })
        .collect();

    let flat_children: Vec<&&str> = children.iter().flat_map(|x| x).collect();
    let root_node = find_root_node(&names, &flat_children);
    println!();
    match root_node {
        Some((i, root)) => {
            println!("Root node is: {}!", root);
            println!();
            println!("Traversing Tree:");
            traverse_tree(&names, &mut weights, &children, i);
        }
        None => println!("No root node identified!"),
    };
}
