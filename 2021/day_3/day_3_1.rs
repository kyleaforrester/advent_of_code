use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut idx_cnts: Vec<(usize, usize)> = Vec::new();
    for line in buffer.split('\n').filter(|x| x.len() > 0) {
        for c_idx in line.chars().rev().enumerate() {
            if c_idx.0 >= idx_cnts.len() {
                idx_cnts.push((0, 0));
            }

            match idx_cnts.get_mut(c_idx.0) {
                Some(c) => match c_idx.1 {
                    '0' => c.0 += 1,
                    '1' => c.1 += 1,
                    _ => panic!("What byte is this? {}", c_idx.1),
                },
                None => panic!("Index {} not in Vec: {:?}", c_idx.0, idx_cnts),
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for counts in idx_cnts.iter().enumerate() {
        if counts.1 .0 >= counts.1 .1 {
            epsilon += 2usize.pow(counts.0 as u32) * 1;
        } else {
            gamma += 2usize.pow(counts.0 as u32) * 1;
        }
    }

    println!("{}", gamma * epsilon);
}
