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
        if self.validate_byr() && self.validate_iyr() && self.validate_eyr() && self.validate_hgt() && self.validate_hcl() && self.validate_ecl() && self.validate_pid() {
            true
        } else {
            false
        }
    }

    fn validate_byr(&self) -> bool {
        match &self.byr {
            Some(s) => match s.parse::<usize>() {
                Ok(i) => {
                    if i >= 1920 && i <= 2002 {
                        true
                    } else {
                        false
                    }
                },
                Err(_e) => false,
            },
            None => false,
        }
    }

    fn validate_iyr(&self) -> bool {
        match &self.iyr {
            Some(s) => match s.parse::<usize>() {
                Ok(i) => {
                    if i >= 2010 && i <= 2020{
                        true
                    } else {
                        false
                    }
                },
                Err(_e) => false,
            },
            None => false,
        }
    }

    fn validate_eyr(&self) -> bool { 
        match &self.eyr {
            Some(s) => match s.parse::<usize>() {
                Ok(i) => {
                    if i >= 2020 && i <= 2030 {
                        true
                    } else {
                        false
                    }
                },
                Err(_e) => false,
            },
            None => false,
        }

    }

    fn validate_hgt(&self) -> bool {
        match &self.hgt {
            Some(s) => {
                match &s[s.len()-2..] {
                    "cm" => {
                        match (&s[..s.len()-2]).parse::<usize>() {
                            Ok(i) => {
                                if i >= 150 && i <= 193 {
                                    true
                                } else {
                                    false
                                }
                            },
                            Err(_e) => false,
                        }
                    },
                    "in" => {
                        match (&s[..s.len()-2]).parse::<usize>() {
                            Ok(i) => {
                                if i >= 59 && i <= 76 {
                                    true
                                } else {
                                    false
                                }
                            },
                            Err(_e) => false,
                        }
                    },
                    _ => false,
                }
            },
            None => false,
        }
    }

    fn validate_hcl(&self) -> bool {
        match &self.hcl {
            Some(s) => {
                if s.len() != 7 || s.chars().nth(0).unwrap() != '#' {
                    false
                } else {
                    s.chars().skip(1).all(|x| x.is_ascii_digit() || x.is_ascii_lowercase())
                }
            }
            None => false,
        }
    }

    fn validate_ecl(&self) -> bool {
        match &self.ecl {
            Some(e) => {
                match e.as_str() {
                    "amb" => true,
                    "blu" => true,
                    "brn" => true,
                    "gry" => true,
                    "grn" => true,
                    "hzl" => true,
                    "oth" => true,
                    _ => false,
                }
            },
            None => false,
        }
    }

    fn validate_pid(&self) -> bool {
        match &self.pid {
            Some(p) => p.len() == 9 && p.chars().all(|x| x.is_ascii_digit()),
            None => false,
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
