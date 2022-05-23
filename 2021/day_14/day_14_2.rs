use std::collections::HashMap;
use std::io::{self, Read};

const STEPS: u32 = 40;

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let input_parts: Vec<&str> = buffer.split("\n\n").collect();

    let mut formulas: HashMap<(char, char), char> = HashMap::new();
    for line in input_parts[1].split('\n').filter(|x| x.len() > 0) {
        let formula_parts: Vec<&str> = line.split(" -> ").collect();
        let mut key_itr = formula_parts[0].chars();
        formulas.insert(
            (key_itr.next().unwrap(), key_itr.next().unwrap()),
            formula_parts[1].chars().nth(0).unwrap(),
        );
    }

    let mut pair_counts: HashMap<(char, char), usize> = HashMap::new();
    for pair in input_parts[0].chars().zip(input_parts[0].chars().skip(1)) {
        let c = pair_counts.entry((pair.0, pair.1)).or_insert(0);
        *c += 1;
    }

    println!("Template: {}", input_parts[0]);
    println!("Before Step 1: {:?}", pair_counts);

    let mut new_pair_counts = HashMap::new();
    for i in 0..STEPS {
        for key in formulas.keys() {
            let new_char = formulas.get(key).unwrap();

            match pair_counts.get(key) {
                Some(v) => {
                    let c = new_pair_counts.entry((key.0, *new_char)).or_insert(0);
                    *c += v;
                    let c = new_pair_counts.entry((*new_char, key.1)).or_insert(0);
                    *c += v;
                }
                None => (),
            }
        }
        pair_counts = new_pair_counts;
        new_pair_counts = HashMap::new();
        println!("After Step {}: {:?}", i + 1, pair_counts);
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for key_val in pair_counts.iter() {
        let c = counts.entry(key_val.0 .0).or_insert(0);
        *c += key_val.1;
        let c = counts.entry(key_val.0 .1).or_insert(0);
        *c += key_val.1;
    }

    // Increment the leading and trailing chars by 1 count
    if let Some(v) = counts.get_mut(&input_parts[0].chars().next().unwrap()) {
        *v += 1;
    }
    if let Some(v) = counts.get_mut(&input_parts[0].chars().next_back().unwrap()) {
        *v += 1;
    }

    let max = counts.iter().max_by_key(|x| x.1).unwrap().1 / 2;
    let min = counts.iter().min_by_key(|x| x.1).unwrap().1 / 2;

    println!("Max: {}, Min: {}, Diff: {}", max, min, max - min);
}
