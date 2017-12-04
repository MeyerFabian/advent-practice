use std::fs::File;
use std::io::Read;

// checks every word against every word
// valid lines are chosen by compare function f
fn valid_words_in_lines(contents: &String, f: fn(&str, &str) -> bool) -> usize {
    let v: Vec<Vec<&str>> = contents
        .lines()
        .map(|s| s.split_whitespace().collect())
        .collect();

    let invalid: usize = v.iter()
        .map(|a| {
            a.iter().enumerate().fold(0, |acc, (i, d)| {
                if acc != 0 {
                    1
                } else {
                    let found = a.iter().skip(i + 1).find(|&c| f(d, c));
                    match found {
                        Some(_) => 1,
                        None => 0,
                    }
                }
            })
        })
        .sum();
    v.len() - invalid
}

// easy compare
fn equals(a: &str, b: &str) -> bool {
    a == b
}

//function for sorting strings
fn sort_string(a: &str) -> String {
    let mut ai = a.chars().collect::<Vec<char>>();
    ai.sort();
    ai.into_iter().collect::<String>()
}

// a string is an anagram of another if they are the same sorted
fn anagram(a: &str, b: &str) -> bool {
    sort_string(a) == sort_string(b)
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let n1 = valid_words_in_lines(&contents, equals);
    println!("valid non double:{}", n1);

    let n2 = valid_words_in_lines(&contents, anagram);
    println!("valid non double non reverse:{}", n2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_test() {
        let contents = "hey hey hey hey
what what is going
blub tar rat"
            .to_string();
        valid_words_in_lines(&contents,equals);
        assert_eq!(1, valid_words_in_lines(&contents, equals));
        assert_eq!(0, valid_words_in_lines(&contents, anagram));
    }
    #[test]
    fn small_test_2() {
        let contents = "hey hey hey hey
what si is going
blub wawr what
murm blood hey mrum
blub bulb
mouse maus raus
pool opol"
            .to_string();
        assert_eq!(6, valid_words_in_lines(&contents, equals));
        assert_eq!(2, valid_words_in_lines(&contents, anagram));
    }
}
