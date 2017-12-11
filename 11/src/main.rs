use std::fs::File;
use std::io::Read;

// The hexagons ("hexes") in this grid are aligned such that adjacent
// hexes can be found to the north, northeast, southeast,
// south, southwest, and northwest:

//       \ n  /
//     nw +--+ ne
//       /    \
//     -+      +-
//       \    /
//     sw +--+ se
//       / s  \

// You have the path the child process took. Starting where he started,
// you need to determine the fewest number of steps required to reach him.
// (A "step" means to move from the hex you are in to any adjacent hex.)

// For example:

//     ne,ne,ne is 3 steps away.
//     ne,ne,sw,sw is 0 steps away (back where you started).
//     ne,ne,s,s is 2 steps away (se,se).
//     se,sw,se,sw,sw is 3 steps away (s,s,sw).

fn shortest_path_alg(words: &[&str]) -> Vec<i32> {
    //PART 1
    // A Hex grid as essentially 3 direction n/s,nw/se and ne/sw
    let mut shortest_path: Vec<i32> = vec![0, 0, 0];

    for word in words {
        match *word {
            // Opposite directions counter each other
            "nw" => shortest_path[0] += 1,
            "se" => shortest_path[0] -= 1,
            "n" => shortest_path[1] += 1,
            "s" => shortest_path[1] -= 1,
            "ne" => shortest_path[2] += 1,
            "sw" => shortest_path[2] -= 1,
            _ => println!("Word {} ignored.", word),
        }
    }
    // There is two adjustments though:

    // Going directly n/s instead of nw/se-ne/sw is one step faster
    // Note: There exists no east and west directions
    // for which this also would hold!
    // We detect this by looking at equal signs of nw and ne direction.
    detect_same_sign(&mut shortest_path, 1, 0, 2);

    // And Secondly:
    // n/s can be combined with se/ne or sw/nw to create ne/se or nw/sw
    // we detect this by different signs of n and se/sw direction
    // order is important! s or n has to come first
    detect_different_sign(&mut shortest_path, 1, 0, 2);
    detect_different_sign(&mut shortest_path, 1, 2, 0);

    shortest_path
}

fn detect_same_sign(shortest_path: &mut [i32], _north: usize, _diag1: usize, _diag2: usize,) {
    let mut i = _diag1;
    let mut j = _diag2;
    let mut ksign = 1;
    if shortest_path[i].signum() == shortest_path[j].signum() {
        let min = (shortest_path[i].abs()).min(shortest_path[j].abs());
        if (shortest_path[i] < 0) {
            ksign = -1;
        }
        shortest_path[i] -= ksign * min;
        shortest_path[j] -= ksign * min;
        shortest_path[_north] += ksign * min;
    }
}

fn detect_different_sign(shortest_path: &mut [i32], _north: usize, _diag1: usize, _diag2: usize) {
    let mut i = _north;
    let mut j = _diag1;
    let mut ksign = 1;
    if shortest_path[i].signum() != shortest_path[j].signum() {
        let min = (shortest_path[i].abs()).min(shortest_path[j].abs());
        if (shortest_path[i] < 0) {
            let temp = i;
            i = j;
            j = temp;
            ksign = -1;
        }
        shortest_path[i] -= min;
        shortest_path[j] += min;
        shortest_path[_diag2] += ksign * min;
    }
}

fn shortest_path_len(shortest_path: &[i32]) -> u32 {
    shortest_path.iter().map(|d| d.abs() as u32).sum()
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let words = contents.split(',').collect::<Vec<&str>>();
    //PART 1
    let vec = shortest_path_alg(&words);
    println!("shortest path {:?}", vec);
    println!("shortest path length {} ", shortest_path_len(&vec));

    // PART 2 may not be the smartest solution to start completely over...
    // but wanted to reuse part1 and to lazy to do find a iterative solution
    // that is probably not too far off from the above
    let mut max = 0;
    for i in 1..words.len() {
        let first_i_words = words.iter().take(i).map(|x| *x).collect::<Vec<&str>>();
        let current = shortest_path_len(&shortest_path_alg(&first_i_words));
        if current > max {
            max = current;
        }
    }
    println!("longest distance of the run {}", max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_path_test_one_dir() {
        let contents = "ne,ne,ne".to_string();
        let words = contents.split(',').collect::<Vec<&str>>();
        assert_eq!(3, shortest_path_len(&shortest_path_alg(&words)));
    }
    #[test]
    fn shortest_path_test_opposed() {
        let contents = "ne,ne,sw,sw".to_string();
        let words = contents.split(',').collect::<Vec<&str>>();
        assert_eq!(0, shortest_path_len(&shortest_path_alg(&words)));
    }
    #[test]
    fn shortest_path_test_combine_different_signs_0() {
        let contents = "ne,ne,s,s".to_string();
        let words = contents.split(',').collect::<Vec<&str>>();
        let shortest_path = shortest_path_alg(&words);
        assert_eq!(-2, shortest_path[0]);
        assert_eq!(0, shortest_path[1]);
        assert_eq!(0, shortest_path[2]);
    }
    #[test]
    fn shortest_path_test_combine_different_signs_1() {
        let contents = "sw,sw,n,n".to_string();
        let words = contents.split(',').collect::<Vec<&str>>();
        let shortest_path = shortest_path_alg(&words);
        assert_eq!(2, shortest_path[0]);
        assert_eq!(0, shortest_path[1]);
        assert_eq!(0, shortest_path[2]);
    }
    #[test]
    fn shortest_path_test_combine_different_signs_2() {
        let contents = "nw,nw,s,s".to_string();
        let words = contents.split(',').collect::<Vec<&str>>();
        let shortest_path = shortest_path_alg(&words);
        assert_eq!(0, shortest_path[0]);
        assert_eq!(0, shortest_path[1]);
        assert_eq!(-2, shortest_path[2]);
    }
    #[test]
    fn shortest_path_test_combine_different_signs_3() {
        let contents = "se,se,n,n".to_string();
        let words = contents.split(',').collect::<Vec<&str>>();
        let shortest_path = shortest_path_alg(&words);
        assert_eq!(0, shortest_path[0]);
        assert_eq!(0, shortest_path[1]);
        assert_eq!(2, shortest_path[2]);
    }
    #[test]
    fn shortest_path_test_combine_same_signs_0() {
        let contents = "se,sw,se,sw,sw".to_string();
        let words = contents.split(',').collect::<Vec<&str>>();
        let shortest_path = shortest_path_alg(&words);
        assert_eq!(0, shortest_path[0]);
        assert_eq!(-2, shortest_path[1]);
        assert_eq!(-1, shortest_path[2]);
    }
    #[test]
    fn shortest_path_test_combine_same_signs_1() {
        let contents = "se,sw,se,sw,se".to_string();
        let words = contents.split(',').collect::<Vec<&str>>();
        let shortest_path = shortest_path_alg(&words);
        assert_eq!(-1, shortest_path[0]);
        assert_eq!(-2, shortest_path[1]);
        assert_eq!(0, shortest_path[2]);
    }
    #[test]
    fn shortest_path_test_combine_same_signs_2() {
        let contents = "ne,nw,ne,nw,nw".to_string();
        let words = contents.split(',').collect::<Vec<&str>>();
        let shortest_path = shortest_path_alg(&words);
        assert_eq!(1, shortest_path[0]);
        assert_eq!(2, shortest_path[1]);
        assert_eq!(0, shortest_path[2]);
    }
    #[test]
    fn shortest_path_test_combine_same_signs_3() {
        let contents = "ne,nw,ne,nw,ne".to_string();
        let words = contents.split(',').collect::<Vec<&str>>();
        let shortest_path = shortest_path_alg(&words);
        assert_eq!(0, shortest_path[0]);
        assert_eq!(2, shortest_path[1]);
        assert_eq!(1, shortest_path[2]);
    }
}
