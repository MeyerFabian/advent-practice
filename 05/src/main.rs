use std::fs::File;
use std::io::Read;

// The message includes a list of the offsets for each jump.
// Jumps are relative: -1 moves to the previous instruction, and 2 skips the next one.
// Start at the first instruction in the list.
// The goal is to follow the jumps until one leads outside the list.

// In addition, these instructions are a little strange;
// after each jump, the offset of that instruction increases by 1.
// So, if you come across an offset of 3, you would move three instructions forward,
// but change it to a 4 for the next time it is encountered.

// For example, consider the following list of jump offsets:

// 0
// 3
// 0
// 1
// -3

// Positive jumps ("forward") move downward; negative jumps move upward.
// For legibility in this example, these offset values will be written all on one line,
// with the current instruction marked in parentheses.
// The following steps would be taken before an exit is found:

//     (0) 3  0  1  -3  - before we have taken any steps.
//     (1) 3  0  1  -3  - jump with offset 0 (that is, don't jump at all).
//                        Fortunately, the instruction is then incremented to 1.
//      2 (3) 0  1  -3  - step forward because of the instruction we just modified.
//                        The first instruction is incremented again, now to 2.
//      2  4  0  1 (-3) - jump all the way to the end; leave a 4 behind.
//      2 (4) 0  1  -2  - go back to where we just were; increment -3 to -2.
//      2  5  0  1  -2  - jump 4 steps forward, escaping the maze.

// In this example, the exit is reached in 5 steps.
// rust doesnt have bidirectional iterators, so we index all :/

fn jump1(v: &mut [i32], i:&mut i32){
    let oldi = *i as usize;
    *i += v[*i as usize];
    v[oldi]+=1;
}

// Now, the jumps are even stranger: after each jump, if the offset was three or more,
// instead decrease it by 1. Otherwise, increase it by 1 as before.
// Using this rule with the above example, the process now takes 10 steps,
// and the offset values after finding the exit are left as 2 3 2 3 -1.
fn jump2(v: &mut [i32], i:&mut i32){
    let oldi = *i as usize;
    *i += v[*i as usize];
    v[oldi]+= if v[oldi] >2 {
        -1
    }else{
        1
    } ;
}

fn run(mut v: &mut [i32], jump: fn(&mut[i32],&mut i32)) -> i32 {
    let mut i: i32 = 0;
    let mut steps =0;
    while i>=0 && i < (v.len() as i32){
        jump(&mut v,&mut i);
        steps+=1;
    }
    steps
}
fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut v: Vec<i32> = contents
        .lines()
        .map(|d| d.parse::<i32>().unwrap())
        .collect();
    let mut v2 = v.clone();
    let steps = run(&mut v, jump1);


    println!("{}", steps);


    let steps2 = run(&mut v2, jump2);
    println!("{}", steps2);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn edge_test() {
        let mut v = vec![0];
        assert_eq!(2, run(&mut v, jump1));
    }
    fn edge_test_2() {
        let mut v = vec![1];
        assert_eq!(1, run(&mut v, jump1));
    }
    #[test]
    fn run_test() {
        let mut v = vec![0,3,0,1,-3];
        assert_eq!(5, run(&mut v, jump1));
        let mut viter = v.iter();
        assert_eq!(Some(&2),viter.next());
        assert_eq!(Some(&5),viter.next());
        assert_eq!(Some(&0),viter.next());
        assert_eq!(Some(&1),viter.next());
        assert_eq!(Some(&-2),viter.next());
    }
    #[test]
    fn edge_test_j2() {
        let mut v = vec![0];
        assert_eq!(2, run(&mut v, jump2));
    }
    fn edge_test_2_j2() {
        let mut v = vec![1];
        assert_eq!(1, run(&mut v, jump2));
    }
    #[test]
    fn run_test_j2() {
        let mut v = vec![0,3,0,1,-3];
        assert_eq!(10, run(&mut v, jump2));
        let mut viter = v.iter();
        assert_eq!(Some(&2),viter.next());
        assert_eq!(Some(&3),viter.next());
        assert_eq!(Some(&2),viter.next());
        assert_eq!(Some(&3),viter.next());
        assert_eq!(Some(&-1),viter.next());
    }
}
