use std::io::{self, Read};

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
    let _my_ticket: Vec<usize> = paragraphs[1].split('\n').nth(1).unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let mut nearby_tickets = Vec::new();
    for line in paragraphs[2].split('\n').skip(1) {
        if line.len() == 0 {
            continue;
        }
        let t: Vec<usize> = line.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        nearby_tickets.push(t);
    }

    let mut sum: usize = 0;
    for t in nearby_tickets {
        sum += t.iter().filter(|&&x| rules.iter().all(|y| !y.test(x))).sum::<usize>();
    }

    println!("{}", sum);
}
