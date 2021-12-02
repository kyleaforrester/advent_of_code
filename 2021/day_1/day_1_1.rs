use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut last_int: Option<usize> = None;
    let mut sum = 0;
    for line in buffer.split('\n') {
        if line.len() == 0 {
            continue;
        }
        let curr_int = line.parse::<usize>().unwrap();
        match last_int {
            Some(l) => {
                if l < curr_int {
                    sum += 1;
                }
            }
            None => (),
        }
        last_int = Some(curr_int);
    }

    println!("{}", sum);
}
