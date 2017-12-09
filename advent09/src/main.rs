use std::fs::File;
use std::io::Read;

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
