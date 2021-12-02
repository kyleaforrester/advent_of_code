use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let lines: Vec<usize> = buffer
        .split('\n')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    let count = (0..lines.len())
        .zip(1..lines.len() - 2)
        .filter(|x| {
            &lines[x.0..x.0 + 3].into_iter().sum::<usize>()
                < &lines[x.1..x.1 + 3].into_iter().sum::<usize>()
        })
        .count();

    println!("{}", count);
}
