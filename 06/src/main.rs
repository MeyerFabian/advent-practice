use std::fs::File;
use std::io::Read;

// In this area, there are sixteen memory banks; each memory bank can hold any number of blocks.
// The goal of the reallocation routine is to balance the blocks between the memory banks.

// The reallocation routine operates in cycles.
// In each cycle, it finds the memory bank with the most blocks
// (ties won by the lowest-numbered memory bank)
// and redistributes those blocks among the banks.
// To do this, it removes all of the blocks from the selected bank,
// then moves to the next (by index) memory bank and inserts one of the blocks.
// It continues doing this until it runs out of blocks;
// if it reaches the last memory bank, it wraps around to the first one.

// The debugger would like to know how many redistributions
// can be done before a blocks-in-banks configuration is produced that has been seen before.

// For example, imagine a scenario with only four memory banks:

//     The banks start with 0, 2, 7, and 0 blocks.
//     The third bank has the most blocks, so it is chosen for redistribution.
//     Starting with the next bank (the fourth bank) and then continuing to the first bank,
//     the second bank, and so on, the 7 blocks are spread out over the memory banks.
//     The fourth, first, and second banks get two blocks each, and the third bank gets one back.
//     The final result looks like this: 2 4 1 2.
//     Next, the second bank is chosen because it contains the most blocks (four).
//     Because there are four memory banks, each gets one block. The result is: 3 1 2 3.
//     Now, there is a tie between the first and fourth memory banks,
//     both of which have three blocks.
//     The first bank wins the tie, and its three blocks are distributed evenly
//     over the other three banks, leaving it with none: 0 2 3 4.
//     The fourth bank is chosen, and its four blocks are distributed
//     such that each of the four banks receives one: 1 3 4 1.
//     The third bank is chosen, and the same thing happens: 2 4 1 2.
// At this point, we've reached a state we've seen before: 2 4 1 2 was already seen.
// The infinite loop is detected after the fifth block redistribution cycle,
// and so the answer in this example is 5.

//Find the maximum i32 in a list, first occurence wins tiebreaker
fn max(v: &[i32]) -> (usize, i32) {
    let a = v.iter()
        .enumerate()
        .rev()
        .max_by(|&(_, x), &(_, y)| x.cmp(y))
        .unwrap();
    (a.0, *a.1)
}

// We iterate only one time over the vector and calc due to length,
// distance and index how mch blocks corresponding index gets.
fn cycle(v: &mut [i32]) {
    let a = max(v);
    v[a.0] = 0;
    let v_len = v.len();
    for (i, elem) in v.iter_mut().enumerate() {
        let new_i = ((i + v_len - a.0 - 1) % v_len) as i32;
        let occ = (a.1 - new_i + v_len as i32 - 1) / v_len as i32;
        *elem += occ;
    }
}

fn reallocation(v: &[i32]) -> (usize, usize) {
    let mut v_collection = vec![v.to_vec()];
    let mut v_clone = v.to_vec();
    let mut step: usize = 0;
    loop {
        step += 1;

        //cycle
        cycle(&mut v_clone);

        //routine to find if step was already employed
        let mut found = true;
        for (ind, row) in v_collection.iter().enumerate() {
            let mut v_c_iter = v_clone.iter();
            found = true;
            for i in row {
                found = found && (Some(i) == v_c_iter.next());
                if !found {
                    break;
                }
            }
            if found {
                //immediately return if we found a reoccurence of state
                return (step, step - ind);
            }
        }
        //push every state in a big vector
        v_collection.push(v_clone.to_vec());
    }
    //error case
    (0, 0)
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let v = contents
        .split("\t")
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let (steps, loopsize) = reallocation(&v);
    println!("steps {} loopsize {}", steps, loopsize);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reall_test() {
        let mut v = vec![0, 2, 7, 0];
        let (steps, loopsize) = reallocation(&v);
        assert_eq!(5, steps);
        assert_eq!(4, loopsize);
    }
    #[test]
    fn cycle_test() {
        let mut v = vec![0, 2, 7, 0];
        cycle(&mut v);
        let mut viter = v.iter();
        assert_eq!(Some(&2), viter.next());
        assert_eq!(Some(&4), viter.next());
        assert_eq!(Some(&1), viter.next());
        assert_eq!(Some(&2), viter.next());
    }
}
