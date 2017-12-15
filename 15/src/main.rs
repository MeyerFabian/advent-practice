use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut generator_inp = contents
        .lines()
        .flat_map(|x| {
            let elem = x.split_whitespace().skip(4).next();
            elem.unwrap().parse::<u32>()
        })
        .collect::<Vec<u32>>();
    println!("gen A {:?}", generator_inp[0]);
    println!("gen b {:?}", generator_inp[1]);


    let mut gen_factors : Vec<u32>= vec![16807,48271];
    let divider = 2147483647;
    let runs = 40000000;
    let mut pair_count = 0;
    let mut generator_copy = generator_inp.to_vec();
    for run in 0..runs{
        for (inp,fac) in generator_copy.iter_mut().zip(gen_factors.iter()){
            *inp= ((*inp as u64)  * (*fac as u64) % divider) as u32;
        }
        let mut gen_copy_iter = generator_copy.iter();
        if *gen_copy_iter.next().unwrap() as u16 == *gen_copy_iter.next().unwrap() as u16{
            pair_count +=1;
        }
    }
    println!("pair_count {}", pair_count);


    let mut generator_copy_2 = generator_inp.to_vec();
    let mut gen_multiples = vec![4,8];
    let mut pair_count2 =0;
    let runs2 = 5000000;
    for run in 0..runs2{
        let mut gen_m_iter = gen_multiples.iter();
        for (inp,fac) in generator_copy_2.iter_mut().zip(gen_factors.iter()){
            let mult = gen_m_iter.next().unwrap();
            //do while loop, executes at least one time
            while{
                *inp= ((*inp as u64)  * (*fac as u64) % divider) as u32;
                (*inp % mult !=0)
            } {}
        }
        let mut gen_copy_2_iter = generator_copy_2.iter();
        if *gen_copy_2_iter.next().unwrap() as u16 == *gen_copy_2_iter.next().unwrap() as u16{
            pair_count2 +=1;
        }
    }
    println!("pair_count2 {}",pair_count2);


}
