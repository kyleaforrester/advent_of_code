use std::cmp::Ordering;
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

    for c_line in lines.iter() {
        let x_step = c_line.start.0.cmp(&c_line.end.0);

        let y_step = c_line.start.1.cmp(&c_line.end.1);

        let mut x = c_line.start.0;
        let mut y = c_line.start.1;

        while x != c_line.end.0 || y != c_line.end.1 {
            match map.get_mut(&(x, y)) {
                Some(v) => *v += 1,
                None => {
                    map.insert((x, y), 1);
                    ()
                }
            }

            x = match x_step {
                Ordering::Greater => x - 1,
                Ordering::Less => x + 1,
                Ordering::Equal => x,
            };
            y = match y_step {
                Ordering::Greater => y - 1,
                Ordering::Less => y + 1,
                Ordering::Equal => y,
            };
        }

        match map.get_mut(&(x, y)) {
            Some(v) => *v += 1,
            None => {
                map.insert((x, y), 1);
                ()
            }
        }
    }

    println!("{}", map.values().filter(|&&x| x >= 2).count());
}
