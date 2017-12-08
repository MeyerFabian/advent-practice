// You receive a signal directly from the CPU.
// Because of your recent assistance with jump instructions,
// it would like you to compute the result of a series of unusual register instructions.

// Each instruction consists of several parts: the register to modify,
// whether to increase or decrease that register's value,
// the amount by which to increase or decrease it, and a condition.
// If the condition fails, skip the instruction without modifying the register.
// The registers all start at 0. The instructions look like this:

// b inc 5 if a > 1
// a inc 1 if b < 5
// c dec -10 if a >= 1
// c inc -20 if c == 10

// These instructions would be processed as follows:

//     Because a starts at 0, it is not greater than 1, and so b is not modified.
//     a is increased by 1 (to 1) because b is less than 5 (it is 0).
//     c is decreased by -10 (to 10) because a is now greater than or equal to 1 (it is 1).
//     c is increased by -20 (to -10) because c is equal to 10.

// After this process, the largest value in any register is 1.

// You might also encounter <= (less than or equal to) or != (not equal to).
// However, the CPU doesn't have the bandwidth to tell you what all the registers are named,
// and leaves that to you to determine.

// What is the largest value in any register after completing the instructions in your puzzle input?
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::cmp::PartialOrd;
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn sub(a: i32, b: i32) -> i32 {
    a - b
}
fn lt(a: i32, b: i32) -> bool {
    a < b
}
fn eq(a: i32, b: i32) -> bool {
    a == b
}
fn neq(a: i32, b: i32) -> bool {
    !(eq(a, b))
}
fn leq(a: i32, b: i32) -> bool {
    lt(a, b) || eq(a, b)
}
fn ge(a: i32, b: i32) -> bool {
    !(lt(a, b))
}
fn gt(a: i32, b: i32) -> bool {
    !(leq(a, b))
}

fn expr_parse<'a>(contents: &'a String, register: &mut HashMap<&'a str, i32>) -> i32 {
    let mut overall_max = 0;
    for line in contents.lines() {
        let split_line = line.split_whitespace()
            .filter(|&d| d != "")
            .collect::<Vec<&str>>();

        let str_a = split_line[0];
        let str_operation = split_line[1];
        let arg_1_1: i32 = split_line[2].parse().unwrap();
        let str_b = split_line[4];
        let str_cmp = split_line[5];
        let arg_2_1: i32 = split_line[6].parse().unwrap();
        let opp = match str_operation {
            "inc" => add,
            _ => sub,
        };

        let cmp = match str_cmp {
            "<" => lt,
            ">" => gt,
            ">=" => ge,
            "<=" => leq,
            "==" => eq,
            _ => neq,
        };

        //add unknown variables to register
        register.entry(str_a).or_insert(0);
        register.entry(str_b).or_insert(0);
        if cmp(*register.get(str_b).unwrap(), arg_2_1) {
            let mut x = register.get_mut(str_a).unwrap();
            let new_val = opp(*x, arg_1_1);
            *x = new_val;
            if (new_val > overall_max) {
                overall_max = new_val
            }
        }
    }
    overall_max
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut register: HashMap<&str, i32> = HashMap::new();
    let overall_max = expr_parse(&contents, &mut register);

    println!(
        "max value: {:?}",
        register
            .iter()
            .max_by(|&(_, val), &(_, val2)| val.cmp(val2))
            .unwrap()
    );
    println!("max value while processing instr. {}", overall_max);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test() {
        let mut contents = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"
            .to_string();
        let mut register: HashMap<&str, i32> = HashMap::new();
        let overall_max = expr_parse(&contents, &mut register);
        let max = register
            .iter()
            .max_by(|&(key, val), &(key2, val2)| val.cmp(val2))
            .unwrap();
        assert_eq!(*max.0, "a");
        assert_eq!(*max.1, 1);
        assert_eq!(overall_max, 10);
    }
}
