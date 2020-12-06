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
        let mut set = HashSet::new();
        for l in s.split('\n') {
            people.push(l.to_string());
            for c in l.chars() {
                set.insert(c);
            }
        }

        return Ok(Group {
            people: people,
            sum: set.len(),
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

