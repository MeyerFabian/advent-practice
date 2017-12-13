extern crate sprs;
use std::fs::File;
use std::io::Read;
use sprs::CsVec;

fn packet_scanner_build(contents: &String) -> CsVec<usize> {
    // Trying out sparse Vectors for fun,
    // could be done just with the vals vector.
    let mut inds = Vec::new();
    let mut vals = Vec::new();
    for _line in contents.lines() {
        let mut line = _line.split(": ").map(|d| d.parse().expect("no integer"));
        inds.push(line.next().expect("index wrong"));
        vals.push(line.next().expect("number wrong"));
    }
    let size = *inds.last().expect("last index") + 1;
    CsVec::new(size, inds, vals)
}

fn severity_packet_scanner(v: &CsVec<usize>, delay: usize) -> usize {
    // The task ahead can be solved by a simple mathematic formula:
    // If index%(2*depth-2)==0 the scanner catches us
    // and we have to add the severity to the severity of our trip.
    // Aadditionally if we have a delay we just delay the index
    // by + delay.
    v.iter().fold(0, |acc, (range, depth)| {
        if (range + delay) % (2 * depth - 2) == 0 {
            acc + range * depth
        } else {
            acc
        }
    })
}

fn caught_packet_scanner(v: &CsVec<usize>, delay: usize) -> bool {
    // We cannot use severity here. Getting caught at the first index
    // doesnt change the sum because range is 0
    v.iter().fold(false, |acc, (range, depth)| {
        if (range + delay) % (2 * depth - 2) == 0 {
            acc || true
        } else {
            acc || false
        }
    })
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let v = packet_scanner_build(&contents);

    //PART 1
    println!("{}", severity_packet_scanner(&v, 0));

    //PART 2
    let mut delay = 0;
    while caught_packet_scanner(&v,delay){
        delay += 1;
    }
    println!("{}", delay);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_scanner() {
        let contents = "0: 3
1: 2
4: 4
6: 4"
            .to_string();
        let v = packet_scanner_build(&contents);
        let sum = severity_packet_scanner(&v, 0);
        assert_eq!(24, sum);
    }
    #[test]
    fn test_packet_scanner_delay() {
        let contents = "0: 3
1: 2
4: 4
6: 4"
            .to_string();
        let v = packet_scanner_build(&contents);
        let sum = severity_packet_scanner(&v, 10);
        assert!(!caught_packet_scanner(&v, 10));
        assert_eq!(0, sum);
    }
}
