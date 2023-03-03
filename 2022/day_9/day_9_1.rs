use std::collections::HashSet;
use std::io::{self, Read};

fn is_adjacent(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut tail = (0, 0);
    let mut head = (0, 0);
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert(tail);
    for line in buffer.split('\n').filter(|x| x.len() > 0) {
        let mut line_iter = line.split(' ');
        let dir = line_iter.next().unwrap().chars().next().unwrap();
        let count = line_iter
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("Could not parse string!");

        for _i in 0..count {
            match dir {
                'U' => {
                    head.1 += 1;
                    if !is_adjacent(&head, &tail) {
                        tail.1 = head.1 - 1;
                        tail.0 = head.0;
                    }
                }
                'D' => {
                    head.1 -= 1;
                    if !is_adjacent(&head, &tail) {
                        tail.1 = head.1 + 1;
                        tail.0 = head.0;
                    }
                }
                'R' => {
                    head.0 += 1;
                    if !is_adjacent(&head, &tail) {
                        tail.0 = head.0 - 1;
                        tail.1 = head.1;
                    }
                }
                'L' => {
                    head.0 -= 1;
                    if !is_adjacent(&head, &tail) {
                        tail.0 = head.0 + 1;
                        tail.1 = head.1;
                    }
                }
                _ => panic!("Invalid direction {}", dir),
            }
            set.insert(tail);
        }
    }

    println!("{}", set.len());
}
