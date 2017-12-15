extern crate advent_shared;
use advent_shared::knothash;
use advent_shared::graphalg;
extern crate bit_vec;
use bit_vec::BitVec;

extern crate itertools;
use itertools::Itertools;

extern crate petgraph;
use petgraph::graphmap::UnGraphMap;

use std::fs::File;
use std::io::Read;
use std::cmp;
// The disk in question consists of a 128x128 grid;
// each square of the grid is either free or used.
// On this disk, the state of the grid is tracked by the bits in a sequence of knot hashes.

// A total of 128 knot hashes are calculated, each corresponding to a single row in the grid;
// each hash contains 128 bits which correspond to individual grid squares.
// Each bit of a hash indicates whether that square is free (0) or used (1).

// The hash inputs are a key string (your puzzle input), a dash,
// and a number from 0 to 127 corresponding to the row.
// For example, if your key string were flqrgnkx,
// then the first row would be given by the bits of the knot hash of flqrgnkx-0,
// the second row from the bits of the knot hash of flqrgnkx-1,
// and so on until the last row, flqrgnkx-127.

// The output of a knot hash is traditionally represented by 32 hexadecimal digits;
// each of these digits correspond to 4 bits, for a total of 4 * 32 = 128 bits.
// To convert to bits, turn each hexadecimal digit to its equivalent binary value,
// high-bit first: 0 becomes 0000, 1 becomes 0001, e becomes 1110, f becomes 1111, and so on;
// a hash that begins with a0c2017...
// in hexadecimal would begin with 10100000110000100000000101110000... in binary.

// Continuing this process, the first 8 rows and columns for key flqrgnkx appear as follows,
// using # to denote used squares, and . to denote free ones:

// ##.#.#..-->
// .#.#.#.#
// ....#.#.
// #.#.##.#
// .##.#...
// ##..#..#
// .#...#..
// ##.#.##.-->
// |      |
// V      V

// In this example, 8108 squares are used across the entire 128x128 grid.

fn create_bitgrid(contents: &String) -> Vec<BitVec> {
    let grid_size = 128;
    let mut key_string = Vec::new();
    let list = (0..256).collect::<Vec<u32>>();
    for row in 0..grid_size {
        key_string.push(contents.to_string() + &format!("-{}", row));
    }
    key_string
        .iter()
        .map(|d| {
            BitVec::from_bytes(&knothash::knot_hash(d, &list)
                .chars()
                .chunks(2)
                .into_iter()
                .map(|chunk| u8::from_str_radix(&chunk.collect::<String>(), 16).unwrap())
                .collect::<Vec<u8>>())
        })
        .collect::<Vec<_>>()
}
// --- Part Two ---

// Now, all the defragmenter needs to know is the number of regions.
// A region is a group of used squares that are all adjacent, not including diagonals.
// Every used square is in exactly one region:
// lone used squares form their own isolated regions,
// while several adjacent squares all count as a single region.

// In the example above, the following nine regions are visible,
// each marked with a distinct digit:

// 11.2.3..-->
//     .1.2.3.4
//     ....5.6.
//     7.8.55.9
//     .88.5...
//     88..5..8
//     .8...8..
//     88.8.88.-->
//     |      |
// V      V

// Of particular interest is the region marked 8;
// while it does not appear contiguous in this small view,
// all of the squares marked 8 are connected when considering the whole 128x128 grid.
// In total, in this example, 1242 regions are present.

// How many regions are present given your key string?

// Your puzzle answer was 1103.

fn region_counting(grid: &Vec<BitVec>) -> usize {
    let mut region_index = 1;
    let mut g = UnGraphMap::new();
    //PART 2
    // 2-pass connected-component_labeling
    let mut grid_labels: Vec<Vec<usize>> = Vec::with_capacity(grid.len());
    let mut labellist: Vec<Vec<usize>> = Vec::new();
    for (iy, y) in grid.iter().enumerate() {
        let mut left_val = 0;
        let mut row = Vec::with_capacity(grid[0].len());
        let mut upper_vec = Vec::new();
        if iy != 0 {
            // This is where you begin to hate rust for not realizing that i
            // actually dont change the data of a row above ever
            upper_vec = grid_labels[iy - 1].to_vec();
        }
        let mut upper_iter = upper_vec.iter();
        for (ix, x) in y.iter().enumerate() {
            let mut upper_val = 0;
            if iy != 0 {
                upper_val = *upper_iter.next().expect("iter out of range");
            }
            let mut this_val = 0;
            if x {
                if upper_val == left_val {
                    if upper_val != 0 {
                        this_val = upper_val;
                    } else {
                        this_val = region_index;
                        labellist.push(vec![this_val]);
                        region_index += 1;
                    }
                } else {
                    if upper_val == 0 {
                        this_val = left_val;
                    } else if left_val == 0 {
                        this_val = upper_val;
                    } else {
                        this_val = upper_val;
                        if !labellist[upper_val - 1].contains(&left_val) {
                            labellist[upper_val - 1].push(left_val);
                        }

                        if !labellist[left_val - 1].contains(&upper_val) {
                            labellist[left_val - 1].push(upper_val);
                        }
                    }
                }
            }
            row.push(this_val);
            left_val = this_val;
        }
        grid_labels.push(row);
    }
    let _ = graphalg::create_ungraphmap(&labellist, &mut g);

    graphalg::group_count(&g)
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let grid_size = 128;
    let grid = create_bitgrid(&contents);

    // PART 1
    let sum: usize = grid.iter().map(|d| d.iter().filter(|x| *x).count()).sum();

    println!("{}", sum);

    let num_groups = region_counting(&grid);
    println!("group count {}", num_groups);
}
