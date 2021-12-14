use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut gen_counts: VecDeque<u64> = VecDeque::new();

    for _i in 0..9 {
        gen_counts.push_back(0);
    }

    for idx in buffer
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
    {
        if let Some(elem) = gen_counts.get_mut(idx) {
            *elem += 1;
        }
    }

    for _day in 0..256 {
        let pregnant = gen_counts[0];
        gen_counts.pop_front();

        if let Some(elem) = gen_counts.get_mut(6) {
            *elem += pregnant;
        }

        gen_counts.push_back(pregnant);
    }

    println!("{}", gen_counts.iter().sum::<u64>());
}
