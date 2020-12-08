use std::io::{self, Read};
use std::collections::HashSet;
use std::collections::VecDeque;

struct Rule {
    outer: String,
    inner: Vec<(u32, String)>,
}

impl Rule {
    fn new(s: &str) -> Result<Rule, String> {
        let s_split: Vec<&str> = s.split(" contain ").collect();
        if s_split.len() != 2 {
            return Err(format!("Rule not in correct format: {}", s));
        }
        let outer = s_split[0].strip_suffix(" bags").unwrap().to_string();

        if s_split[1] == "no other bags." {
            return Ok(Rule {outer: outer, inner: Vec::new()});
        }

        let mut inner: Vec<(u32, String)> = Vec::new();
        for bag in s_split[1].strip_suffix('.').unwrap().split(',') {
            let b = match bag.strip_suffix(" bags") {
                Some(s) => s,
                None => bag.strip_suffix(" bag").unwrap(),
            };

            let tokens: Vec<&str>  = match b.strip_prefix(' ') {
                Some(s) => s.split(' ').collect(),
                None => b.split(' ').collect(),
            };

            let count = match tokens[0].parse::<u32>() {
                Ok(c) => c,
                Err(_s) => return Err(format!("Not an integer: {}", tokens[0])),
            };
            inner.push((count, format!("{} {}", tokens[1], tokens[2])));
        }

        return Ok(Rule {
            outer: outer,
            inner: inner,
        });
    }
}

fn possible_containers(rules: &Vec<Rule>, bag: &str) -> HashSet<String> {
    let mut queue = VecDeque::new();
    let mut containers = HashSet::new();
    queue.push_back(bag.to_string());

    while queue.len() > 0 {
        let b = queue.pop_front().unwrap();
        for r in rules.iter().filter(|x| x.inner.iter().any(|y| y.1 == b)) {
            containers.insert(r.outer.clone());
            queue.push_back(r.outer.clone());
        }
    }

    containers
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut rules: Vec<Rule> = Vec::new();
    for line in buffer.split('\n') {
        match Rule::new(line) {
            Ok(r) => rules.push(r),
            Err(_s) => (),
        }
    }

    println!("{}", possible_containers(&rules, "shiny gold").len());
}

