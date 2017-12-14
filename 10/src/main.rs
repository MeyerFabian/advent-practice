extern crate advent_shared;
use std::fs::File;
use std::io::Read;
use advent_shared::knothash;
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
    let shuffled_list = knothash::shuffle(&list, &lengh_input, 1);
    println!("Check: {}", shuffled_list[0] * shuffled_list[1]);

    //PART 2
    let hex_string = knothash::knot_hash(&contents, &list);

    println!("Hex-String: {}", hex_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle_test_0() {
        let list = (0..5).collect::<Vec<u32>>();
        let length_input = vec![3, 4, 1, 5];
        let v = knothash::shuffle(&list, &length_input, 1);
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
        let dense_hash = knothash::dense_hashing(&sparse_hash, 1);
        assert_eq!(64, dense_hash[0]);
    }
    #[test]
    fn bit_to_hex_test_0() {
        let bit = vec![64, 7, 255];
        assert_eq!("4007ff", knothash::bit_vector_to_hex_string(&bit));
    }
    #[test]
    fn knot_hash_test_0() {
        let contents = "".to_string();
        let list = (0..256).collect::<Vec<u32>>();
        assert_eq!(
            "a2582a3a0e66e6e86e3812dcb672a272",
            knothash::knot_hash(&contents, &list)
        );
    }
    #[test]
    fn knot_hash_test_1() {
        let contents = "AoC 2017".to_string();
        let list = (0..256).collect::<Vec<u32>>();
        assert_eq!(
            "33efeb34ea91902bb2f59c9920caa6cd",
            knothash::knot_hash(&contents, &list)
        );
    }

    #[test]
    fn knot_hash_test_2() {
        let contents = "1,2,3".to_string();
        let list = (0..256).collect::<Vec<u32>>();
        assert_eq!(
            "3efbe78a8d82f29979031a4aa0b16a9d",
            knothash::knot_hash(&contents, &list)
        );
    }

    #[test]
    fn knot_hash_test_3() {
        let contents = "1,2,4".to_string();
        let list = (0..256).collect::<Vec<u32>>();
        assert_eq!(
            "63960835bcdc130f0b66d7ff4f6a5a8e",
            knothash::knot_hash(&contents, &list)
        );
    }

}
