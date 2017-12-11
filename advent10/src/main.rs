#![feature(slice_rotate)]

use std::fs::File;
use std::io::Read;
// To achieve this, begin with a list of numbers from 0 to 255,
// a current position which begins at 0 (the first element in the list),
// a skip size (which starts at 0), and a sequence of lengths (your puzzle input).
// Then, for each length:

//     Reverse the order of that length of elements in the list,
//     starting with the element at the current position.
//     Move the current position forward by that length plus the skip size.
//     Increase the skip size by one.

// The list is circular;
// if the current position and the length try to reverse elements beyond the end of the list,
// the operation reverses using as many extra elements as it needs from the front of the list.
// If the current position moves past the end of the list, it wraps around to the front.
// Lengths larger than the size of the list are invalid.

// Here's an example using a smaller list:

// Suppose we instead only had a circular list containing five elements,
// 0, 1, 2, 3, 4, and were given input lengths of 3, 4, 1, 5.

//     The list begins as [0] 1 2 3 4
//     (where square brackets indicate the current position).
//     The first length, 3, selects([0] 1 2) 3 4
//     (where parentheses indicate the sublist to be reversed).
//     After reversing that section (0 1 2 into 2 1 0), we get ([2] 1 0) 3 4.
//     Then, the current position moves forward by the length
//     , 3, plus the skip size, 0: 2 1 0 [3] 4. Finally, the skip size increases to 1.

//     The second length, 4, selects a section which wraps: 2 1) 0 ([3] 4.
//     The sublist 3 4 2 1 is reversed to form 1 2 4 3: 4 3) 0 ([1] 2.
//     The current position moves forward by the length plus the skip size,
//     a total of 5, causing it not to move because it wraps around:
//     4 3 0 [1] 2. The skip size increases to 2.

//     The third length, 1, selects a sublist of a single element,
//     and so reversing it has no effect.
//     The current position moves forward by the length (1)
//     plus the skip size (2): 4 [3] 0 1 2. The skip size increases to 3.

//     The fourth length, 5, selects every element starting with the second:
//     4) ([3] 0 1 2. Reversing this sublist (3 0 1 2 4 into 4 2 1 0 3)
//     produces: 3) ([4] 2 1 0.
//     Finally, the current position moves forward by 8: 3 4 2 1 [0].
//     The skip size increases to 4.

// In this example, the first two numbers in the list end up being 3 and 4;
// to check the process, you can multiply them together to produce 12.

// However, you should instead use the standard list size of 256 (with values 0 to 255)
// and the sequence of lengths in your puzzle input. Once this process is complete,
// what is the result of multiplying the first two numbers in the list?

// Instead of merely running one round like you did above,
// run a total of 64 rounds, using the same length sequence in each round.
// The current position and skip size should be preserved between rounds.
// For example, if the previous example was your first round,
// you would start your second round with the same length sequence
// (3, 4, 1, 5, 17, 31, 73, 47, 23,),
// but start with the previous round's current position (4) and skip size (4).
fn shuffle(list: &[u32], length_input: &[u32], rounds: usize) -> Vec<u32> {
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

fn dense_hashing(hash: &[u32], dense_hash_len: usize) -> Vec<u32> {
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
fn bit_vector_to_hex_string(bit_vector: &[u32]) -> String {
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

fn knot_hash(contents: &String, list: &[u32]) -> String {
    let mut file_ascii = File::open("input_ascii.txt").expect("Unable to open");
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
fn main() {
    let list = (0..256).collect::<Vec<u32>>();
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    //PART 1
    let lengh_input = if contents.len() > 0 {
        contents
            .split(',')
            .map(|d| d.parse().unwrap())
            .collect::<Vec<u32>>()
    } else {
        Vec::new()
    };
    let shuffled_list = shuffle(&list, &lengh_input, 1);
    println!("Check: {}", shuffled_list[0] * shuffled_list[1]);

    //PART 2
    let hex_string = knot_hash(&contents, &list);

    println!("Hex-String: {}", hex_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle_test_0() {
        let list = (0..5).collect::<Vec<u32>>();
        let length_input = vec![3, 4, 1, 5];
        let v = shuffle(&list, &length_input, 1);
        assert_eq!(3, v[0]);
        assert_eq!(4, v[1]);
        assert_eq!(2, v[2]);
        assert_eq!(1, v[3]);
        assert_eq!(0, v[4]);
        assert_eq!(12, v[0] * v[1]);
    }
    #[test]
    fn dense_hash_test_0() {
        let sparse_hash = [65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
        let dense_hash = dense_hashing(&sparse_hash, 1);
        assert_eq!(64, dense_hash[0]);
    }
    #[test]
    fn bit_to_hex_test_0() {
        let bit = vec![64, 7, 255];
        assert_eq!("4007ff", bit_vector_to_hex_string(&bit));
    }
    #[test]
    fn knot_hash_test_0() {
        let contents = "".to_string();
        let list = (0..256).collect::<Vec<u32>>();
        assert_eq!(
            "a2582a3a0e66e6e86e3812dcb672a272",
            knot_hash(&contents, &list)
        );
    }
    #[test]
    fn knot_hash_test_1() {
        let contents = "AoC 2017".to_string();
        let list = (0..256).collect::<Vec<u32>>();
        assert_eq!(
            "33efeb34ea91902bb2f59c9920caa6cd",
            knot_hash(&contents, &list)
        );
    }

    #[test]
    fn knot_hash_test_2() {
        let contents = "1,2,3".to_string();
        let list = (0..256).collect::<Vec<u32>>();
        assert_eq!(
            "3efbe78a8d82f29979031a4aa0b16a9d",
            knot_hash(&contents, &list)
        );
    }

    #[test]
    fn knot_hash_test_3() {
        let contents = "1,2,4".to_string();
        let list = (0..256).collect::<Vec<u32>>();
        assert_eq!(
            "63960835bcdc130f0b66d7ff4f6a5a8e",
            knot_hash(&contents, &list)
        );
    }



}
