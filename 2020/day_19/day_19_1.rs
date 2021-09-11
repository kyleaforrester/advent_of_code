use std::collections::HashMap;
use std::io::{self, Read};

struct Rule {
    id: usize,
    logic: Logic,
}

enum Logic {
    Or { lh: Box<Logic>, rh: Box<Logic> },
    And(Vec<Logic>),
    Ref(usize),
    Char(char),
}

impl Rule {
    fn new(string: &str) -> Rule {
        let splits: Vec<&str> = string.split(':').collect();
        Rule {
            id: splits[0].parse::<usize>().unwrap(),
            logic: Logic::new(splits[1].trim()),
        }
    }

    fn is_match(
        &self,
        string: &str,
        rules: &Vec<Rule>,
        cache: &mut HashMap<(String, usize), bool>,
    ) -> bool {
        //println!("Evaluating Rule {} for {}", self.id, string);
        match cache.get(&(string.to_string(), self.id)) {
            Some(b) => return *b,
            None => (),
        }

        let retval = self.logic.is_match(string, rules, cache);

        cache.insert((string.to_string(), self.id), retval);
        //println!("Rule {} match {}: {}", self.id, string, retval);
        retval
    }
}

impl Logic {
    fn new(string: &str) -> Logic {
        if string.contains('|') {
            let split: Vec<&str> = string.split('|').collect();
            Logic::Or {
                lh: Box::new(Logic::new(split[0].trim())),
                rh: Box::new(Logic::new(split[1].trim())),
            }
        } else if string.contains(' ') {
            let children: Vec<Logic> = string.split(' ').map(|x| Logic::new(x)).collect();
            Logic::And(children)
        } else if string.contains('"') {
            Logic::Char(string.chars().nth(1).unwrap())
        } else {
            Logic::Ref(string.parse::<usize>().unwrap())
        }
    }

    fn is_match(
        &self,
        string: &str,
        rules: &Vec<Rule>,
        cache: &mut HashMap<(String, usize), bool>,
    ) -> bool {
        match self {
            Logic::Char(c) => string == c.to_string(),
            Logic::Ref(r) => rules
                .iter()
                .filter(|x| x.id == *r)
                .nth(0)
                .unwrap()
                .is_match(string, rules, cache),
            Logic::Or { lh, rh } => {
                if lh.is_match(string, rules, cache) || rh.is_match(string, rules, cache) {
                    return true;
                }
                false
            }
            Logic::And(c) => {
                if string.len() <= 1 {
                    return false;
                }
                match c.len() {
                    2 => {
                        for i in 1..string.len() {
                            if c[0].is_match(&string[..i], rules, cache)
                                && c[1].is_match(&string[i..], rules, cache)
                            {
                                return true;
                            }
                        }
                        false
                    }
                    3 => {
                        for i in 1..string.len() {
                            if c[0].is_match(&string[..i], rules, cache) {
                                for j in i + 1..string.len() {
                                    if c[1].is_match(&string[i..j], rules, cache)
                                        && c[2].is_match(&string[j..], rules, cache)
                                    {
                                        return true;
                                    }
                                }
                            }
                        }
                        false
                    }
                    _ => panic!("Undefined number of children for AND operation"),
                }
            }
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let mut rules = Vec::new();
    let mut cache = HashMap::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let sections: Vec<&str> = buffer.split("\n\n").collect();

    // Populate the rules
    for line in sections[0].split('\n') {
        if line.len() == 0 {
            continue;
        }
        rules.push(Rule::new(line));
    }

    // Sort the rules
    rules.sort_unstable_by_key(|x| x.id);

    // Evaluate the matches
    let count = sections[1]
        .split('\n')
        .filter(|x| x.len() > 0 && rules[0].is_match(x, &rules, &mut cache))
        .count();
    println!("{}", count);
}
