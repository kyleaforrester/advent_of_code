use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut horiz = 0;
    let mut depth = 0;

    for line in buffer.split('\n').filter(|x| x.len() > 0) {
        let elems: Vec<&str> = line.split(' ').collect();
        let distance = &elems[1]
            .parse::<i32>()
            .expect(&format!("Not a valid integer: {}", &elems[1]));

        match elems[0] {
            "forward" => horiz += distance,
            "down" => depth += distance,
            "up" => depth -= distance,
            _ => panic!("Not a valid command: {}", &elems[0]),
        }
    }

    println!("{}", horiz * depth);
}
