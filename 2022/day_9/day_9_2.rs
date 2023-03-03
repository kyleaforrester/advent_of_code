use std::cmp::Ordering;
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

    let mut snake: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert((0, 0));
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
                    snake[0].1 += 1;
                    for i in 1..snake.len() {
                        if !is_adjacent(&snake[i - 1], &snake[i]) {
                            match snake[i].1.cmp(&snake[i - 1].1) {
                                Ordering::Less => snake[i].1 += 1,
                                Ordering::Greater => snake[i].1 -= 1,
                                Ordering::Equal => (),
                            }
                            match snake[i].0.cmp(&snake[i - 1].0) {
                                Ordering::Less => snake[i].0 += 1,
                                Ordering::Greater => snake[i].0 -= 1,
                                Ordering::Equal => (),
                            }
                        }
                    }
                }
                'D' => {
                    snake[0].1 -= 1;
                    for i in 1..snake.len() {
                        if !is_adjacent(&snake[i - 1], &snake[i]) {
                            match snake[i].1.cmp(&snake[i - 1].1) {
                                Ordering::Less => snake[i].1 += 1,
                                Ordering::Greater => snake[i].1 -= 1,
                                Ordering::Equal => (),
                            }
                            match snake[i].0.cmp(&snake[i - 1].0) {
                                Ordering::Less => snake[i].0 += 1,
                                Ordering::Greater => snake[i].0 -= 1,
                                Ordering::Equal => (),
                            }
                        }
                    }
                }
                'R' => {
                    snake[0].0 += 1;
                    for i in 1..snake.len() {
                        if !is_adjacent(&snake[i - 1], &snake[i]) {
                            match snake[i].1.cmp(&snake[i - 1].1) {
                                Ordering::Less => snake[i].1 += 1,
                                Ordering::Greater => snake[i].1 -= 1,
                                Ordering::Equal => (),
                            }
                            match snake[i].0.cmp(&snake[i - 1].0) {
                                Ordering::Less => snake[i].0 += 1,
                                Ordering::Greater => snake[i].0 -= 1,
                                Ordering::Equal => (),
                            }
                        }
                    }
                }
                'L' => {
                    snake[0].0 -= 1;
                    for i in 1..snake.len() {
                        if !is_adjacent(&snake[i - 1], &snake[i]) {
                            match snake[i].1.cmp(&snake[i - 1].1) {
                                Ordering::Less => snake[i].1 += 1,
                                Ordering::Greater => snake[i].1 -= 1,
                                Ordering::Equal => (),
                            }
                            match snake[i].0.cmp(&snake[i - 1].0) {
                                Ordering::Less => snake[i].0 += 1,
                                Ordering::Greater => snake[i].0 -= 1,
                                Ordering::Equal => (),
                            }
                        }
                    }
                }
                _ => panic!("Invalid direction {}", dir),
            }
            set.insert(snake[snake.len() - 1]);
        }
    }

    println!("{}", set.len());
}
