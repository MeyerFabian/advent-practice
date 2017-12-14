extern crate advent_shared;
use advent_shared::knothash;
use std::fs::File;
use std::io::Read;
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
    let sum: u32 = key_string
        .iter()
        .map(|d| {
            knothash::knot_hash(d, &list)
                .chars()
                .map(|d| u8::from_str_radix(&d.to_string(), 16).unwrap().count_ones())
                .sum::<u32>()
        })
        .sum();

    println!("{}", sum);
}
