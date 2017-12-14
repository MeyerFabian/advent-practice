extern crate advent_shared;
use advent_shared::knothash;

extern crate bit_vec;
use bit_vec::BitVec;

extern crate itertools;
use itertools::Itertools;

use std::fs::File;
use std::io::Read;
use std::cmp;
fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let grid_size = 128;
    let mut key_string = Vec::new();
    let list = (0..256).collect::<Vec<u32>>();
    for row in 0..grid_size {
        key_string.push(contents.to_string() + &format!("-{}", row));
    }
    //PART 1
    //We really should use chunks instead of single chars here or sth. to make this more efficient

    let grid = key_string
        .iter()
        .map(|d| {
            BitVec::from_bytes(&knothash::knot_hash(d, &list)
                .chars()
                .chunks(2)
                .into_iter()
                .map(|chunk| u8::from_str_radix(&chunk.collect::<String>(), 16).unwrap())
                .collect::<Vec<u8>>())
        })
        .collect::<Vec<_>>();
    let sum :usize= grid.iter().map(|d| d.iter().filter(|x| *x).count()).sum();

    let mut region_index =1;

    //PART 2
    // puh this will take to much time
    // i will omit this part because i know how to it
    // and the whole algorithm can be seen on wikipedia anyway
    // https://en.wikipedia.org/wiki/Connected-component_labeling
    println!("{}", sum);
    println!("{}",grid[0][0]);
}
