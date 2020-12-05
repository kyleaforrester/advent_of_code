use std::io::{self, Read};
use std::convert::TryFrom;

struct Ticket {
    row: u32,
    seat: u32,
    seat_id: u32,
}

impl Ticket {
    fn new(s: &str) -> Result<Ticket, String> {
        if s.len() != 10 {
            return Err(format!("Ticket length must be 10.  Given ticket: {}", s))
        }

        let mut row = 0;
        for c in s.chars().take(7).enumerate() {
            match c.1 {
                'F' => (),
                'B' => row += 2u32.pow(6 - u32::try_from(c.0).unwrap()),
                _ => return Err(format!("Invalid row character in ticket: {}", s)),
            }
        }

        let mut seat = 0;
        for c in s.chars().skip(7).enumerate() {
            match c.1 {
                'L' => (),
                'R' => seat += 2u32.pow(2 - u32::try_from(c.0).unwrap()),
                _ => return Err(format!("Invalid seat character in ticket: {}", s)),
            }
        }

        let seat_id = row * 8 + seat;

        Ok(Ticket {
            row: row,
            seat: seat,
            seat_id: seat_id,
        })
    }
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut tickets: Vec<Ticket> = Vec::new();
    for line in buffer.split('\n') {
        match Ticket::new(line) {
            Ok(t) => tickets.push(t),
            Err(_s) => (),
        }
    }

    let highest = tickets.iter().max_by_key(|x| x.seat_id).unwrap();

    println!("{}", highest.seat_id);
}

