use std::io::{self, Read};
use std::iter::FromIterator;

enum TState {
    Initial,
    Digit,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Token {
    OpenP,
    ClosedP,
    Number(i64),
    Add,
    Mult,
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut state = TState::Initial;
    let mut tokens = Vec::new();
    let mut num_builder = Vec::new();
    for c in s.chars() {
        match state {
            TState::Initial => {
                if c.is_ascii_digit() {
                    num_builder.push(c);
                    state = TState::Digit;
                } else if c == '*' {
                    tokens.push(Token::Mult);
                } else if c == '+' {
                    tokens.push(Token::Add);
                } else if c == '(' {
                    tokens.push(Token::OpenP);
                } else if c == ')' {
                    tokens.push(Token::ClosedP);
                }
            },
            TState::Digit=> {
                if c.is_ascii_digit() {
                    num_builder.push(c);
                } else {
                    tokens.push(Token::Number(String::from_iter(num_builder.clone()).parse::<i64>().unwrap()));
                    num_builder.clear();
                    if c == '*' {
                        tokens.push(Token::Mult);
                    } else if c == '+' {
                        tokens.push(Token::Add);
                    } else if c == '(' {
                        tokens.push(Token::OpenP);
                    } else if c == ')' {
                        tokens.push(Token::ClosedP);
                    }
                    state = TState::Initial;
                }
            },
        }
    }
    if num_builder.len() > 0 {
        tokens.push(Token::Number(String::from_iter(num_builder.clone()).parse::<i64>().unwrap()));
    }
    tokens
}

fn reverse_polish_notation(tokens: Vec<Token>) -> Vec<Token> {
    let mut output_stack = Vec::new();
    let mut operator_stack = Vec::new();
    
    for t in tokens.into_iter() {
        match t {
            Token::OpenP => operator_stack.push(t),
            Token::ClosedP => {
                while operator_stack.last().unwrap() != &Token::OpenP {
                    output_stack.push(operator_stack.pop().unwrap());
                }
                if operator_stack.len() == 0 {
                    panic!("Mismatch parenthesis!");
                }
                // Removes the open parenthesis
                operator_stack.pop().unwrap();
            },
            Token::Number(_i) => output_stack.push(t),
            Token::Add | Token::Mult => {
                while operator_stack.len() > 0 && operator_stack.last().unwrap() != &Token::OpenP {
                    output_stack.push(operator_stack.pop().unwrap());
                }
                operator_stack.push(t);
            }
        }
    }
    while operator_stack.len() > 0 {
        output_stack.push(operator_stack.pop().unwrap());
    }
    output_stack
}


fn evaluate(tokens: &Vec<Token>) -> i64 {
    let mut stack: Vec<i64> = Vec::new();

    for t in tokens {
        match t {
            Token::Number(i) => stack.push(*i),
            Token::Add => {
                let first = stack.pop().unwrap();
                let second = stack.pop().unwrap();
                stack.push(first + second);
            },
            Token::Mult => {
                let first = stack.pop().unwrap();
                let second = stack.pop().unwrap();
                stack.push(first * second);
            },
            _ => panic!(format!("Invalid polish notation: {:?}", tokens)),
        }
    }
    stack.pop().unwrap()
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut sum = 0;
    for line in buffer.split('\n') {
        if line.len() == 0 {
            continue;
        }
        sum += evaluate(&reverse_polish_notation(tokenize(line)));
    }

    println!("{}", sum);
}

