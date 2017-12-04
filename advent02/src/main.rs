use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
fn main() {
    let file = File::open("checksum.txt").expect("Unable to open");
    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut sum2 = 0;
    for line in reader.lines() {
        let split: Vec<i32> = line.unwrap()
            .split("\t")
            .map(|d| d.parse().unwrap())
            .collect();
        sum += sum_max_min(&split);
        sum2 += sum_even_divided(&split);
    }

    println!{"{}",sum};

    println!{"{}",sum2};
}

// The spreadsheet consists of rows of apparently-random numbers.
// To make sure the recovery process is on the right track,
// they need you to calculate the spreadsheet's checksum.
// For each row, determine the difference between the largest value and the smallest value;
// the checksum is the sum of all of these differences.

//     For example, given the following spreadsheet:

//     5 1 9 5
//     7 5 3
//     2 4 6 8

//     The first row's largest and smallest values are 9 and 1, and their difference is 8.
//     The second row's largest and smallest values are 7 and 3, and their difference is 4.
//     The third row's difference is 6.

//     In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.

fn sum_max_min(data: &[i32]) -> i32 {
    return data.iter().max().unwrap() - data.iter().min().unwrap();
}

// It sounds like the goal is to find the only two numbers in each row
// where one evenly divides the other - that is,
// where the result of the division operation is a whole number.
// They would lie you to find those numbers on each line, divide them,
// and add up each line's result.

//     For example, given the following spreadsheet:

// 5 9 2 8
//     9 4 7 3
//     3 8 6 5

//     In the first row, the only two numbers that evenly divide are 8 and 2;
//     the result of this division is 4.
//     In the second row, the two numbers are 9 and 3; the result is 3.
//     In the third row, the result is 2.

//     In this example, the sum of the results would be 4 + 3 + 2 = 9.

fn sum_even_divided(data: &[i32]) -> i32 {
    let mut sum2 = 0;
    data.iter()
        .enumerate()
        .map(|(i, a)| {
            data.iter().skip(i + 1).fold(0, |acc, c| {
                let mut  v = 0;
                if c % a == 0 {
                    v = c / a
                }
                if a % c == 0 {
                    v = a / c
                };
                acc + v
            })
        })
        .sum()
}
