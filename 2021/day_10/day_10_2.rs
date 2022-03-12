use std::io::{self, Read};

enum Chunk {
    Incomplete(Vec<char>),
    Corrupted(char),
    Complete,
}

impl Chunk {
    fn new(string: &str) -> Chunk {
        let mut stack = Vec::new();

        for c in string.chars() {
            match c {
                ')' => match stack.pop() {
                    Some(p) => {
                        if p != '(' {
                            return Chunk::Corrupted(c);
                        }
                    }
                    None => return Chunk::Corrupted(c),
                },
                ']' => match stack.pop() {
                    Some(p) => {
                        if p != '[' {
                            return Chunk::Corrupted(c);
                        }
                    }
                    None => return Chunk::Corrupted(c),
                },
                '}' => match stack.pop() {
                    Some(p) => {
                        if p != '{' {
                            return Chunk::Corrupted(c);
                        }
                    }
                    None => return Chunk::Corrupted(c),
                },
                '>' => match stack.pop() {
                    Some(p) => {
                        if p != '<' {
                            return Chunk::Corrupted(c);
                        }
                    }
                    None => return Chunk::Corrupted(c),
                },
                _ => stack.push(c),
            }
        }
        if stack.len() > 0 {
            Chunk::Incomplete(stack)
        } else {
            Chunk::Complete
        }
    }

    fn autocomplete_score(&self) -> Option<u64> {
        match self {
            Chunk::Incomplete(v) => {
                let mut score = 0;
                for c in v.iter().rev() {
                    score *= 5;
                    score += match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Invalid stack element: {}", c),
                    };
                }
                println!("stack: {:?}, score: {}", v, score);
                Some(score)
            }
            Chunk::Corrupted(_c) => None,
            Chunk::Complete => None,
        }
    }
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let chunks: Vec<Chunk> = buffer
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| Chunk::new(x))
        .collect();

    let mut scores: Vec<u64> = chunks
        .iter()
        .filter_map(|x| x.autocomplete_score())
        .collect();
    scores.sort_unstable();
    let score = scores[scores.len() / 2];

    println!("{}", score);
}
