use std::fs::File;
use std::io::Read;
// You sit for a while and record part of the stream (your puzzle input).
// The characters represent groups - sequences that begin with { and end with }.
// Within a group, there are zero or more other things, separated by commas:
// either another group or garbage. Since groups can contain other groups, a }
// only closes the most-recently-opened unclosed group - that is, they are nestable.
// Your puzzle input represents a single, large group which itself contains many smaller ones.

// Sometimes, instead of a group, you will find garbage. Garbage begins with < and ends with >.
// Between those angle brackets, almost any character can appear, including { and }.
// Within garbage, < has no special meaning.

// In a futile attempt to clean up the garbage,
// some program has canceled some of the characters within it using !:inside garbage,
// any character that comes after ! should be ignored, including <, >, and even another !.

// You don't see any characters that deviate from these rules.
// Outside garbage, you only find well-formed groups,
// and garbage always terminates according to the rules above.

// Here are some self-contained pieces of garbage:

//     <>, empty garbage.
//     <random characters>, garbage containing random characters.
//     <<<<>, because the extra < are ignored.
//     <{!>}>, because the first > is canceled.
//     <!!>, because the second ! is canceled, allowing the > to terminate the garbage.
//     <!!!>>, because the second ! and the first > are canceled.
//     <{o"i!a,<{i<a>, which ends at the first >.

// Here are some examples of whole streams and the number of groups they contain:

//     {}, 1 group.
//     {{{}}}, 3 groups.
//     {{},{}}, also 3 groups.
//     {{{},{},{{}}}}, 6 groups.
//     {<{},{},{{}}>}, 1 group (which itself contains garbage).
//     {<a>,<a>,<a>,<a>}, 1 group.
//     {{<a>},{<a>},{<a>},{<a>}}, 5 groups.
//     {{<!>},{<!>},{<!>},{<a>}}, 2 groups (since all but the last > are canceled).

// Your goal is to find the total score for all groups in your input.
// Each group is assigned a score which is one more than the score of the group
// that immediately contains it. (The outermost group gets a score of 1.)

//     {}, score of 1.
//     {{{}}}, score of 1 + 2 + 3 = 6.
//     {{},{}}, score of 1 + 2 + 2 = 5.
//     {{{},{},{{}}}}, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
//     {<a>,<a>,<a>,<a>}, score of 1.
//     {{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
//     {{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
//     {{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.

// What is the total score for all groups in your input?

// Now, you're ready to remove the garbage.

//     To prove you've removed it,
//     you need to count all of the characters within the garbage.
//     The leading and trailing < and > don't count,
//     nor do any canceled characters or the ! doing the canceling.

//     <>, 0 characters.
//     <random characters>, 17 characters.
//     <<<<>, 3 characters.
//        <{!>}>, 2 characters.
//        <!!>, 0 characters.
//        <!!!>>, 0 characters.
//       <{o"i!a,<{i<a>, 10 characters.

// How many non-canceled characters are within the garbage in your puzzle input?
fn score_parse(contents: &String) -> (u32, u32) {
    let mut depth = 0;
    let mut score = 0;
    let mut char_iter = contents.chars();
    let mut escaped = false;
    let mut garbage_cntr = 0;
    let mut garbage = false;
    for c in char_iter {
        if escaped {
            escaped = false;
        } else if c == '!' {
            escaped = true;
        } else if garbage {
            if c == '>' {
                garbage = false;
            } else {
                garbage_cntr += 1;
            }
        } else {
            match c {
                '{' => {
                    depth += 1;
                }
                '}' => {
                    score += depth;
                    depth -= 1;
                }
                '<' => {
                    garbage = true;
                }
                _ => {}
            };
        }
    }
    (score, garbage_cntr)
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let (score, garb_cnt) = score_parse(&contents);
    println!("score is {}", score);
    println!("garbage cnt is {}", garb_cnt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_test_0() {
        let contents = "{}".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(1, score);
    }

    #[test]
    fn score_test_1() {
        let contents = "{{{}}}".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(6, score);
    }

    #[test]
    fn score_test_2() {
        let contents = "{{},{}}".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(5, score);
    }

    #[test]
    fn score_test_3() {
        let contents = "{{{},{},{{}}}}".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(16, score);
    }

    #[test]
    fn score_test_4() {
        let contents = "{<a>,<a>,<a>,<a>}".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(1, score);
    }

    #[test]
    fn score_test_5() {
        let contents = "{{<ab>},{<ab>},{<ab>},{<ab>}}".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(9, score);
    }

    #[test]
    fn score_test_6() {
        let contents = "{{<!!>},{<!!>},{<!!>},{<!!>}}".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(9, score);
    }

    #[test]
    fn score_test_7() {
        let contents = "{{<a!>},{<a!>},{<a!>},{<ab>}}".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(3, score);
    }
    #[test]
    fn garbage_test_0() {
        let contents = "<>".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(0, garb_cnt);
    }
    #[test]
    fn garbage_test_1() {
        let contents = "<random characters>".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(17, garb_cnt);
    }
    #[test]
    fn garbage_test_2() {
        let contents = "<<<<>".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(3, garb_cnt);
    }
    #[test]
    fn garbage_test_3() {
        let contents = "<{!>}>".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(2, garb_cnt);
    }
    #[test]
    fn garbage_test_4() {
        let contents = "<!!>".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(0, garb_cnt);
    }
    #[test]
    fn garbage_test_5() {
        let contents = "<!!!>>".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(0, garb_cnt);
    }
    #[test]
    fn garbage_test_6() {
        let contents = "<{o\"i!a,<{i<a>".to_string();
        let (score, garb_cnt) = score_parse(&contents);
        assert_eq!(10, garb_cnt);
    }
}
