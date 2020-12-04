use std::io::{self, Read};

struct Passport {
    ecl: Option<String>,
    pid: Option<String>,
    eyr: Option<String>,
    hcl: Option<String>,
    byr: Option<String>,
    iyr: Option<String>,
    cid: Option<String>,
    hgt: Option<String>,
}

impl Passport {
    fn new(s: &str) -> Passport {
        let (mut ecl, mut pid, mut eyr, mut hcl, mut byr, mut iyr, mut cid, mut hgt) = (None, None, None, None, None, None, None,None);
        for l in s.split(char::is_whitespace) {
            if l.len() < 4 {
                continue;
            }
            match &l[..4] {
                "ecl:" => ecl = Some(String::from(&l[4..])),
                "pid:" => pid = Some(String::from(&l[4..])),
                "eyr:" => eyr = Some(String::from(&l[4..])),
                "hcl:" => hcl = Some(String::from(&l[4..])),
                "byr:" => byr = Some(String::from(&l[4..])),
                "iyr:" => iyr = Some(String::from(&l[4..])),
                "cid:" => cid = Some(String::from(&l[4..])),
                "hgt:" => hgt = Some(String::from(&l[4..])),
                _ => (),
            }
        }

        Passport {
            ecl: ecl,
            pid: pid,
            eyr: eyr,
            hcl: hcl,
            byr: byr,
            iyr: iyr,
            cid: cid,
            hgt: hgt,
        }
    }

    fn is_valid(&self) -> bool {
        if self.ecl.is_some() && self.pid.is_some() && self.eyr.is_some() && self.hcl.is_some() && self.byr.is_some() && self.iyr.is_some() && self.hgt.is_some() {
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut passports: Vec<Passport> = Vec::new();
    for line in buffer.split("\n\n") {
        passports.push(Passport::new(line));
    }

    let valid_count = passports.iter().filter(|x| x.is_valid()).count();

    println!("{}", valid_count);
}
