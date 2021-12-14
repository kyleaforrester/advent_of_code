use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut fish: Vec<u32> = buffer
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    for _day in 0..80 {
        let mut pregnant = 0;
        for f in fish.iter_mut() {
            if *f == 0 {
                *f = 6;
                pregnant += 1;
            } else {
                *f -= 1;
            }
        }

        for _new in 0..pregnant {
            fish.push(8);
        }
    }

    println!("{}", fish.len());
}
