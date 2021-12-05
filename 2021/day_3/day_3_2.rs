use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let numbers: Vec<String> = buffer
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| x.to_string())
        .collect();

    let mut idx = 0;
    let mut oxy_numbers = numbers.clone();
    while oxy_numbers.len() > 1 && idx < numbers[0].len() {
        let ones = oxy_numbers
            .iter()
            .filter(|x| x.chars().nth(idx).unwrap() == '1')
            .count();
        let zeroes = oxy_numbers
            .iter()
            .filter(|x| x.chars().nth(idx).unwrap() == '0')
            .count();
        if ones >= zeroes {
            oxy_numbers = oxy_numbers
                .drain(..)
                .filter(|x| x.chars().nth(idx).unwrap() == '1')
                .collect();
        } else {
            oxy_numbers = oxy_numbers
                .drain(..)
                .filter(|x| x.chars().nth(idx).unwrap() == '0')
                .collect();
        }
        idx += 1;
    }

    let mut idx = 0;
    let mut co2_numbers = numbers.clone();
    while co2_numbers.len() > 1 && idx < numbers[0].len() {
        let ones = co2_numbers
            .iter()
            .filter(|x| x.chars().nth(idx).unwrap() == '1')
            .count();
        let zeroes = co2_numbers
            .iter()
            .filter(|x| x.chars().nth(idx).unwrap() == '0')
            .count();
        if ones < zeroes {
            co2_numbers = co2_numbers
                .drain(..)
                .filter(|x| x.chars().nth(idx).unwrap() == '1')
                .collect();
        } else {
            co2_numbers = co2_numbers
                .drain(..)
                .filter(|x| x.chars().nth(idx).unwrap() == '0')
                .collect();
        }
        idx += 1;
    }

    println!(
        "{}",
        usize::from_str_radix(&co2_numbers[0], 2).unwrap()
            * usize::from_str_radix(&oxy_numbers[0], 2).unwrap()
    );
}
