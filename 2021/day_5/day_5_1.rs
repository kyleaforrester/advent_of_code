use std::cmp;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

impl Line {
    fn new(string: &str) -> Line {
        let points: Vec<&str> = string.split(" -> ").collect();

        let first_point: Vec<usize> = points[0]
            .split(',')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();
        let second_point: Vec<usize> = points[1]
            .split(',')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        Line {
            start: (first_point[0], first_point[1]),
            end: (second_point[0], second_point[1]),
        }
    }
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let lines: Vec<Line> = buffer
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| Line::new(x))
        .collect();

    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    for c_line in lines
        .iter()
        .filter(|x| x.start.0 == x.end.0 || x.start.1 == x.end.1)
    {
        let small_x = cmp::min(c_line.start.0, c_line.end.0);
        let large_x = cmp::max(c_line.start.0, c_line.end.0);
        let small_y = cmp::min(c_line.start.1, c_line.end.1);
        let large_y = cmp::max(c_line.start.1, c_line.end.1);

        for x in small_x..large_x + 1 {
            for y in small_y..large_y + 1 {
                match map.get_mut(&(x, y)) {
                    Some(v) => *v += 1,
                    None => {
                        map.insert((x, y), 1);
                        ()
                    }
                }
            }
        }
    }

    println!("{}", map.values().filter(|&&x| x >= 2).count());
}
