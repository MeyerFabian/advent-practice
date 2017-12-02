use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("captcha.txt").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

   let vec: Vec<_> = contents.chars().map(|d| d.to_digit(10).unwrap()).collect();
    captcha1(&vec);
}

// You notice a progress bar that jumps to 50% completion. Apparently, the door isn't yet satisfied, but it did emit a star as encouragement. The instructions change:

// Now, instead of considering the next digit, it wants you to consider the digit halfway around the circular list.
// That is, if your list contains 10 items, only include a digit in your sum if the digit 10/2 = 5 steps forward matches it. Fortunately, your list has an even number of elements.

fn captcha2(vec: &Vec<u32>){
    let mut sum = 0;
    for (ai,bi) in vec.iter().take(vec.len()/2).zip(vec.iter().cycle().skip(vec.len()/2)){
        if ai==bi {
            sum+=2*ai;
        }
    }
    println!("Captcha Code is: {}!",sum)
}

// The captcha requires you to review a sequence of digits (your puzzle input) and find the sum of all digits that match the next digit in the list.
// The list is circular, so the digit after the last digit is the first digit in the list.

//     For example:

//     1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the third digit (2) matches the fourth digit.
//     1111 produces 4 because each digit (all 1) matches the next.
//     1234 produces 0 because no digit matches the next.
//     91212129 produces 9 because the only digit that matches the next one is the last digit, 9.

fn captcha1(vec: &Vec<u32>){
    let mut pre = vec.last().unwrap();
    let mut sum = 0;
    for (ai,bi) in vec.iter().zip(vec.iter().cycle().skip(1)){
        if ai==bi {
            sum+=ai;
        }
    }
    println!("Captcha Code is: {}!",sum);
}
