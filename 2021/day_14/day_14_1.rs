use std::collections::HashMap;
use std::io::{self, Read};
use std::iter::FromIterator;

const STEPS: u32 = 10;

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let input_parts: Vec<&str> = buffer.split("\n\n").collect();

    let mut polymer: Vec<char> = input_parts[0].chars().map(|c| c.clone()).collect();

    let mut formulas = HashMap::new();
    for line in input_parts[1].split('\n').filter(|x| x.len() > 0) {
        let formula_parts: Vec<&str> = line.split(" -> ").collect();
        formulas.insert(
            formula_parts[0].to_string(),
            formula_parts[1].chars().nth(0).unwrap(),
        );
    }

    println!("Template: {}", String::from_iter(polymer.iter()));

    for i in 0..STEPS {
        let mut new_polymer = Vec::new();

        let mut pair = String::new();
        for p in polymer.iter() {
            pair.push(*p);
            if pair.len() > 2 {
                pair = pair[1..].to_string();
            }

            match formulas.get(&pair) {
                Some(f) => new_polymer.push(f.clone()),
                None => (),
            }

            new_polymer.push(p.clone());
        }
        polymer = new_polymer;
        println!(
            "After step {}: {}",
            i + 1,
            String::from_iter(polymer.iter())
        );
    }

    let mut counts = HashMap::new();
    for c in polymer.iter() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    let max = counts.iter().max_by_key(|x| x.1).unwrap().1;
    let min = counts.iter().min_by_key(|x| x.1).unwrap().1;

    println!("{}", max - min);
}
