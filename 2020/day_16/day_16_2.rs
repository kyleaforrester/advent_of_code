use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Rule {
    name: String,
    ranges: Vec<(usize, usize)>,
}

impl Rule {
    fn new(s: &str) -> Rule {
        let split: Vec<&str> = s.split(": ").collect();
        let mut ranges: Vec<(usize, usize)> = Vec::new();
        for r_str in split[1].split(" or ") {
            let r: Vec<usize> = r_str.split('-').map(|x| x.parse::<usize>().unwrap()).collect();
            ranges.push((r[0], r[1]));
        }

        Rule {
            name: split[0].to_string(),
            ranges: ranges,
        }
    }

    fn test(&self, n: usize) -> bool {
        self.ranges.iter().any(|x| x.0 <= n && n <= x.1)
    }
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let paragraphs: Vec<&str> = buffer.split("\n\n").collect();

    let rules: Vec<Rule> = paragraphs[0].split('\n').map(|x| Rule::new(x)).collect();
    let my_ticket: Vec<usize> = paragraphs[1].split('\n').nth(1).unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let mut nearby_tickets = Vec::new();
    for line in paragraphs[2].split('\n').skip(1) {
        if line.len() == 0 {
            continue;
        }
        let t: Vec<usize> = line.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        nearby_tickets.push(t);
    }

    let mut valid_tickets = Vec::new();
    for t in nearby_tickets {
        let is_valid = t.iter().all(|&n| rules.iter().any(|r| r.test(n)));
        if is_valid {
            valid_tickets.push(t.clone());
        }
    }

    let mut possible_rules: Vec<Vec<Rule>> = Vec::new();
    for i in 0..my_ticket.len() {
        let nums: Vec<usize> = valid_tickets.iter().map(|t| t[i]).collect();
        let p_rules: Vec<Rule> = rules.iter().filter(|r| nums.iter().all(|&n| r.test(n))).map(|r| r.clone()).collect();
        possible_rules.push(p_rules);
    }

    let mut indexed_rules: Vec<(usize, Rule)> = Vec::new();
    while possible_rules.iter().any(|p| p.len() > 0) {
        // find index with only 1 possibility
        let solo_inds_rules: Vec<(usize, Rule)> = possible_rules.iter().enumerate().filter(|p| p.1.len() == 1).map(|p| (p.0, (p.1[0]).clone())).collect();
        for solo in solo_inds_rules.iter() {
            // Delete rule with that possibility from all possible rules
            for p_rules in possible_rules.iter_mut() {
                match p_rules.iter().position(|p| p.name == solo.1.name) {
                    Some(u) => {
                        p_rules.remove(u);
                        ()
                    },
                    None => (),
                }
            }
            // Add the indexed rule to our list
            indexed_rules.push((solo.0, solo.1.clone()));
        }
    }

    let indices: Vec<usize> = indexed_rules.iter().filter(|rt| rt.1.name.contains("departure")).map(|rt| rt.0).collect();
    let product: usize = my_ticket.iter().enumerate().filter(|nt| indices.contains(&nt.0)).fold(1, |acc, mt| acc * mt.1);

    println!("{}", product);
}
