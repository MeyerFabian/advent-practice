#![feature(slice_rotate)]
pub mod knothash{
use std::fs::File;
use std::io::Read;

pub fn shuffle(list: &[u32], length_input: &[u32], rounds: usize) -> Vec<u32> {
    let mut vec = list.to_vec();
    let mut pos = 0;
    let mut skip_size = 0;
    for _ in 0..rounds {
        for length in length_input {
            // List gets divided into two lists the part that gets reversed and rest
            let len = *length as usize;
            let list_take = vec.iter()
                .cycle()
                .skip(pos)
                .take(len)
                .map(|x| *x)
                .collect::<Vec<u32>>();
            let rev = list_take.iter().rev();
            let rev_len = rev.len();
            let mut new_vec = vec.iter()
                .cycle()
                .skip(pos + len)
                .take(vec.len() - rev_len)
                .map(|x| *x)
                .collect::<Vec<u32>>();
            // Then we fuse the lists together again
            new_vec.extend(rev);

            // And rotate the lists elements to get the initial list indexes back
            // which got shifted due to:
            // - new_vec.extend(rev);
            // - and skipping first pos indices and cycling
            new_vec.rotate((2 * vec.len() - (rev_len + pos)) % vec.len());
            vec = new_vec.clone();

            pos = (pos + len + skip_size) % vec.len();
            skip_size += 1;
        }
    }
    vec
}
// Once the rounds are complete, you will be left with the numbers from 0 to 255 in some order,
// called the sparse hash.
// Your next task is to reduce these to a list of only 16 numbers called the dense hash.
// To do this, use numeric bitwise XOR to combine each consecutive block of 16 numbers
// in the sparse hash (there are 16 such blocks in a list of 256 numbers).
// So, the first element in the dense hash is the first sixteen elements
// of the sparse hash XOR'd together,
// the second element in the dense hash is the second sixteen elements
// of the sparse hash XOR'd together, etc.

//     For example, if the first sixteen elements of your sparse hash are as shown below,
//     and the XOR operator is ^, you would calculate the first output number like this:

// 65 ^ 27 ^ 9 ^ 1 ^ 4 ^ 3 ^ 40 ^ 50 ^ 91 ^ 7 ^ 6 ^ 0 ^ 2 ^ 5 ^ 68 ^ 22 = 64

// Perform this operation on each of the sixteen blocks of sixteen numbers
// in your sparse hash to determine the sixteen numbers in your dense hash

pub fn dense_hashing(hash: &[u32], dense_hash_len: usize) -> Vec<u32> {
    let mut dense_hash = Vec::new();
    let mut hash_iter = hash.iter();
    let elements_per_entry = hash.len() / dense_hash_len;

    for _ in 0..dense_hash_len {
        let mut entry = *hash_iter.next().unwrap();
        for _ in 0..(elements_per_entry - 1) {
            entry = entry ^ (*hash_iter.next().unwrap());
        }
        dense_hash.push(entry);
    }
    dense_hash
}
// Finally, the standard way to represent a Knot Hash is as a single hexadecimal string;
// the final output is the dense hash in hexadecimal notation.
// Because each number in your dense hash will be between 0 and 255 (inclusive),
// always represent each number as two hexadecimal digits (including a leading zero as necessary).
// So, if your first three numbers are 64, 7, 255, they correspond to the hexadecimal numbers
// 40, 07, ff, and so the first six characters of the hash would be 4007ff.
// Because every Knot Hash is sixteen such numbers,
// the hexadecimal representation is always 32 hexadecimal digits (0-f) long.
pub fn bit_vector_to_hex_string(bit_vector: &[u32]) -> String {
    bit_vector
        .iter()
        .map(|d| {
            let hex = format!("{:x}", d);

            if hex.len() == 1 {
                "0".to_string() + &hex
            } else {
                hex
            }
        })
        .collect::<String>()
}

// First, from now on, your input should be taken not as a list of numbers, but as a string of bytes instead.
// Unless otherwise specified, convert characters to bytes using their ASCII codes.
// This will allow you to handle arbitrary ASCII strings,
// and it also ensures that your input lengths are never larger than 255.
// For example, if you are given 1,2,3,
// you should convert it to the ASCII codes for each character: 49,44,50,44,51.

// Once you have determined the sequence of lengths to use,
// add the following lengths to the end of the sequence: 17, 31, 73, 47, 23.
// For example, if you are given 1,2,3, your final sequence of lengths should be
// 49,44,50,44,51,17,31,73,47,23
// (the ASCII codes from the input string combined with the standard length suffix values).

pub fn knot_hash(contents: &String, list: &[u32]) -> String {
    let mut file_ascii = File::open("../advent_shared/input_ascii.txt").expect("Unable to open");
    let mut contents_ascii = String::new();
    file_ascii.read_to_string(&mut contents_ascii);
    let mut length_input_ascii = contents.chars().map(|d| d as u32).collect::<Vec<u32>>();
    if contents_ascii.len() > 0 {
        let length_input_ext = contents_ascii
            .split(',')
            .map(|d| d.parse().unwrap())
            .collect::<Vec<u32>>();
        length_input_ascii.extend(length_input_ext.iter());
    }

    let sparse_hash = shuffle(&list, &length_input_ascii, 64);

    let dense_hash = dense_hashing(&sparse_hash, 16);

    let hex_string = bit_vector_to_hex_string(&dense_hash);

    hex_string
}
}