use std::io::{self, Read};
use std::convert::TryFrom;

enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

struct Machine {
    program: Vec<Op>,
    ptr: usize,
    acc: i32,
    executed: Vec<usize>,
}

impl Op {
    fn new(s: &str) -> Result<Op, String> {
        if s.len() < 5 {
            return Err(format!("Op too short: {}", s));
        }

        match &s[0..3] {
            "nop" => {
                match &s[4..].parse::<i32>() {
                    Ok(i) => return Ok(Op::Nop(*i)),
                    Err(_e) => return Err(format!("Nop could not parse integer {}", &s[4..])),
                }
            },
            "acc" => {
                match &s[4..].parse::<i32>() {
                    Ok(i) => return Ok(Op::Acc(*i)),
                    Err(_e) => return Err(format!("Acc could not parse integer {}", &s[4..])),
                }
            },
            "jmp" => {
                match &s[4..].parse::<i32>() {
                    Ok(i) => return Ok(Op::Jmp(*i)),
                    Err(_e) => return Err(format!("Jmp could not parse integer {}", &s[4..])),
                }
            },
            _ => return Err(format!("{} not a valid Op code", &s[0..3])),
        }
    }
}



impl Machine {
    fn new(s: &str) -> Machine {
        let mut program = Vec::new();
        for line in s.split('\n') {
            match Op::new(line) {
                Ok(o) => program.push(o),
                Err(_e) => (),
            }
        }

        Machine {
            program: program,
            ptr: 0,
            acc: 0,
            executed: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.ptr = 0;
        self.acc = 0;
        self.executed = Vec::new();
    }

    fn run(&mut self) -> Result<i32, String> {
        while !self.executed.contains(&self.ptr) {
            self.executed.push(self.ptr);

            let op = match self.program.get(self.ptr) {
                Some(o) => o,
                None => return Err(format!("Invalid ptr: {}", self.ptr)),
            };
            match op {
                Op::Nop(_i) => self.ptr += 1,
                Op::Acc(i) => {
                    self.acc += i;
                    self.ptr += 1;
                },
                Op::Jmp(i) => {
                    match usize::try_from((self.ptr as i32) + i) {
                        Ok(o) => self.ptr = o,
                        Err(_e) => return Err(format!("Jumping {} from ptr {} is not valid", i, self.ptr)),
                    }
                },
            }

            if self.ptr == self.program.len() {
                return Ok(self.acc);
            }
        }

        Err(format!("Machine will not exit, loop detected at ptr {}", self.ptr))
    }

    fn fix(&mut self) -> Result<i32, String> {
        let jmps: Vec<(usize, i32)> = self.program.iter().enumerate().filter(|x| match x.1 { Op::Jmp(_i) => true, _ => false, }).map(|x| match x.1 { Op::Jmp(i) => (x.0, *i), _ => panic!(),}).collect();
        let nops: Vec<(usize, i32)> = self.program.iter().enumerate().filter(|x| match x.1 { Op::Nop(_i) => true, _ => false, }).map(|x| match x.1 { Op::Nop(i) => (x.0, *i), _ => panic!(),}).collect();

        for jmp in jmps {
            self.program.remove(jmp.0);
            self.program.insert(jmp.0, Op::Nop(jmp.1));

            match self.run() {
                Ok(i) => return Ok(i),
                Err(_e) => self.reset(),
            }

            self.program.remove(jmp.0);
            self.program.insert(jmp.0, Op::Jmp(jmp.1));
        }

        for nop in nops {
            self.program.remove(nop.0);
            self.program.insert(nop.0, Op::Jmp(nop.1));

            match self.run() {
                Ok(i) => return Ok(i),
                Err(_e) => self.reset(),
            }

            self.program.remove(nop.0);
            self.program.insert(nop.0, Op::Nop(nop.1));
        }

        Err("Cannot fix Machine".to_string())
    }
}





fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut machine = Machine::new(&buffer);

    match machine.fix() {
        Ok(i) => println!("{}", i),
        Err(e) => println!("{}", e),
    }
}
