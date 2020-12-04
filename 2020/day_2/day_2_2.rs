use std::io::{self, Read};
use std::num::ParseIntError;

struct Password {
    password: String,
    policy: Policy,
}

struct Policy {
    substr: String,
    low: usize,
    high: usize,
}

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).expect("Cannot read from stdin");

    let mut passwords = Vec::new();
    for line in input.split('\n') {
        match Password::new(line) {
            Ok(p) => passwords.push(p),
            Err(e) => continue,
        }
    }

    let valid_passwords = passwords.iter().filter(|x| x.is_valid()).count();

    println!("{}", valid_passwords);
}


impl Password {
    fn new(string: &str) -> Result<Password, String> {
        let strings: Vec<&str> = string.split(' ').collect();

        let indices: Vec<&str> = strings[0].split('-').collect();

        if strings.len() < 3 || indices.len() != 2 {
            return Err(format!("Password String not parsable: {}", string));
        }
        println!("{:?}", strings);

        let high;
        let low;
        match indices[0].parse() {
            Ok(l) => low = l,
            Err(e) => return Err(format!("Low policy not an integer: {}", indices[0])),
        }
        match indices[1].parse() {
            Ok(h) => high = h,
            Err(e) => return Err(format!("High policy not an integer: {}", indices[1])),
        }
        let p = Policy {
            substr: String::from(&(strings[1])[..1]),
            low: low,
            high: high,
        };

        Ok(Password {
            password: String::from(strings[2]),
            policy: p,
        })
    }

    fn is_valid(&self) -> bool {
        let i_1 = self.password.chars().nth(self.policy.low - 1).unwrap();
        let i_2 = self.password.chars().nth(self.policy.high - 1).unwrap();

        let s = self.policy.substr.chars().nth(0).unwrap();

        (i_1 == s && i_2 != s) || (i_1 != s && i_2 == s)
    }
}
