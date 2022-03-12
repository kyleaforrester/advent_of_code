use std::io::{self, Read};

fn invalid_char(string: &str) -> Option<char> {
    let mut stack = Vec::new();

    for c in string.chars() {
        match c {
            ')' => match stack.pop() {
                Some(p) => {
                    if p != '(' {
                        return Some(c);
                    }
                }
                None => return Some(c),
            },
            ']' => match stack.pop() {
                Some(p) => {
                    if p != '[' {
                        return Some(c);
                    }
                }
                None => return Some(c),
            },
            '}' => match stack.pop() {
                Some(p) => {
                    if p != '{' {
                        return Some(c);
                    }
                }
                None => return Some(c),
            },
            '>' => match stack.pop() {
                Some(p) => {
                    if p != '<' {
                        return Some(c);
                    }
                }
                None => return Some(c),
            },
            _ => stack.push(c),
        }
    }
    None
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let invalid_chars: Vec<char> = buffer
        .split('\n')
        .filter(|x| x.len() > 0)
        .filter_map(|x| invalid_char(x))
        .collect();

    let score = invalid_chars
        .iter()
        .map(|x| match x {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Invalid ending char: {}", x),
        })
        .sum::<u32>();

    println!("{}", score);
}
