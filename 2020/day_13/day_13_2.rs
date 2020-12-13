use std::io::{self, Read};
use std::convert::TryFrom;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut buses: Vec<(usize, u64)> = Vec::new();
    let lines: Vec<&str> = buffer.split('\n').collect();
    for bus_str in lines[1].split(',').enumerate() {
        let bus: u64 = match bus_str.1.parse() {
            Ok(n) => n,
            Err(_e) => continue,
        };
        buses.push((bus_str.0, bus));
    }

    println!("{}", find_timestamp(&buses));

}

fn find_timestamp(buses: &Vec<(usize, u64)>) -> u64 {
    let mut multiple = buses[0].1;
    let mut accumulator = 0;
    for tup in buses.iter().skip(1) {
        while (accumulator + u64::try_from(tup.0).unwrap()) % tup.1 != 0 {
            accumulator += multiple;
        }
        multiple = multiple * tup.1;
    }
    accumulator
}

