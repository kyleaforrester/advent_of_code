use std::io::{self, Read};
use std::collections::HashSet;

struct Group {
    people: Vec<String>,
    sum: usize,
}

impl Group {
    fn new(s: &str) -> Result<Group, String> {
        if s.len() == 0 {
            return Err("Empty string for Group".to_string());
        }

        let mut people = Vec::new();
        let mut sets = Vec::new();
        for l in s.split('\n') {
            if l.len() == 0 {
                continue;
            }
            people.push(l.to_string());
            let mut set = HashSet::new();
            for c in l.chars() {
                set.insert(c);
            }
            sets.push(set);
        }

        let sum = sets.iter().fold(sets[0].clone(), |acc, x| acc.intersection(&x).copied().collect()).len();

        return Ok(Group {
            people: people,
            sum: sum,
        });
    }
}


fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut groups: Vec<Group> = Vec::new();
    for text in buffer.split("\n\n") {
        match Group::new(text) {
            Ok(g) => groups.push(g),
            Err(_s) => (),
        }
    }

    let sum = groups.iter().fold(0, |acc, x| acc + x.sum);

    println!("{}", sum);
}

