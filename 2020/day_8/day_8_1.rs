use std::io::{self, Read};
use std::convert::TryFrom;

enum Op {
    Nop,
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
            "nop" => return Ok(Op::Nop),
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

    fn run(&mut self) -> Result<i32, String> {
        while !self.executed.contains(&self.ptr) {
            self.executed.push(self.ptr);

            let op = match self.program.get(self.ptr) {
                Some(o) => o,
                None => return Err(format!("Invalid ptr: {}", self.ptr)),
            };
            match op {
                Op::Nop => self.ptr += 1,
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

            if self.ptr >= self.program.len() {
                return Err(format!("Program completed successfully!"));
            }
        }

        Ok(self.acc)
    }
}





fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut machine = Machine::new(&buffer);

    match machine.run() {
        Ok(i) => println!("{}", i),
        Err(e) => println!("{}", e),
    }
}
