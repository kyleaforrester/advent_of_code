use std::io::{self, Read};

enum Side {
    Left,
    Right,
}

struct Ship {
    x: i32,
    y: i32,
    w_x: i32,
    w_y: i32,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            w_x: 10,
            w_y: 1,
        }
    }

    fn turn(&mut self, side: Side, amount: i32) {
        let mut clockwise_turns = (amount % 360) / 90;
        match side {
            Side::Left => {
                if clockwise_turns == 1 {
                    clockwise_turns = 3;
                } else if clockwise_turns == 3 {
                    clockwise_turns = 1;
                }
            },
            Side::Right => (),
        }
        match clockwise_turns {
            0 => (),
            1 => {
                let temp = self.w_x;
                self.w_x = self.w_y;
                self.w_y = -temp;
            },
            2 => {
                self.w_x = -self.w_x;
                self.w_y = -self.w_y;
            },
            3 => {
                let temp = self.w_x;
                self.w_x = -self.w_y;
                self.w_y = temp;
            },
            _ => panic!(format!("Clockwise_turns cannot be {}", clockwise_turns)),
        }
    }

    fn parse_and_make_moves(&mut self, s: &str) {
        for mov in s.split('\n') {
            if mov.len() == 0 {
                continue;
            }
            let mode = match mov.chars().nth(0) {
                Some(i) => i,
                None => panic!(format!("Invalid mode: {}", mov)),
            };
            let amount = match mov.chars().skip(1).collect::<String>().parse::<i32>() {
                Ok(i) => i,
                Err(_e) => panic!(format!("Invalid amount: {}", mov)),
            };

            match mode {
                'N' => self.w_y += amount,
                'S' => self.w_y -= amount,
                'E' => self.w_x += amount,
                'W' => self.w_x -= amount,
                'L' => self.turn(Side::Left, amount),
                'R' => self.turn(Side::Right, amount),
                'F' => {
                    self.x += self.w_x * amount;
                    self.y += self.w_y * amount;
                },
                _ => panic!(format!("Invalid mode: {}", mode)),
            }
        }
    }
}


fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut ship = Ship::new();

    ship.parse_and_make_moves(&buffer);

    println!("{}", ship.x.abs() + ship.y.abs());
}
