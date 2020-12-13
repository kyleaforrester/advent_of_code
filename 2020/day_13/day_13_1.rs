use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut buses = Vec::new();
    let lines: Vec<&str> = buffer.split('\n').collect();
    let start_time: usize = lines[0].parse().expect("Start time must be a number");
    for bus_str in lines[1].split(',') {
        let bus: usize = match bus_str.parse() {
            Ok(n) => n,
            Err(_e) => continue,
        };
        buses.push(bus);
    }

    let (first_bus, wait_time) = buses.iter().map(|x| wait_time(*x, start_time)).min_by_key(|x| x.1).unwrap();

    println!("{}", first_bus * wait_time);

}

fn wait_time(bus: usize, start_time: usize) -> (usize, usize) {
    let wait_time = bus - (start_time % bus);
    (bus, wait_time)
}
