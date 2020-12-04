use std::io::{self, Read};
use std::convert::TryFrom;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut items: Vec<i64> = Vec::new();
    for line in buffer.split('\n') {
        let mut item = 0;
        match line.parse() {
            Ok(n) => item = n,
            Err(n) => continue,
        }
        items.push(item);
    }

    let comps = find_components(&items, 3, 2020).unwrap();

    let product: i64 = comps.iter().fold(1, |acc, x| acc * x);

    println!("{}", product);
}

fn find_components(items: &[i64], count: i64, sum: i64) -> Option<Vec<i64>> {
    if count == 0 {
        if sum == 0 {
            return Some(Vec::new())
        } else {
            return None
        }
    }

    for tup in items.iter().take(usize::try_from(i64::try_from(items.len()).expect("Long length") - (count - 1)).expect("Too many components")).enumerate() {
        let mut comps;
        match find_components(&items[tup.0 + 1..], count - 1, sum - tup.1) {
            Some(c) => comps = c,
            None => continue,
        }
        comps.push(*tup.1);
        return Some(comps)
    }

    None
}





