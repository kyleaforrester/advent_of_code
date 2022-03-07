use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let count: usize = buffer
        .trim()
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| {
            x.split(" | ")
                .nth(1)
                .unwrap()
                .split(' ')
                .filter(|d| [2, 3, 4, 7].contains(&d.len()))
                .count()
        })
        .sum();

    println!("{}", count);
}
