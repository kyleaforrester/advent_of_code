use std::io::{self, Read};
use std::convert::TryFrom;

#[derive(Clone, Copy)]
enum Compass {
    East,
    West,
    North,
    South,
}

enum Side {
    Left,
    Right,
}

struct Ship {
    x: i32,
    y: i32,
    direction: Compass,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            direction: Compass::East,
        }
    }

    fn turn(&mut self, side: Side, amount: i32) {
        let dirs = [Compass::North, Compass::East, Compass::South, Compass::West];
        let self_index = match self.direction {
            Compass::North => 0,
            Compass::East => 1,
            Compass::South => 2,
            Compass::West => 3,
        };
        let displacement = (amount % 360) / 90;

        // Rust % operator is the remainder, not the modulo
        // This can return negative numbers
        // Therefore instead of using a % b, I use:
        // (((a % b) + b) % b)
        // This is a common workaround in languages where % is the remainder
        self.direction = match side {
            Side::Left => {
                dirs[usize::try_from((((self_index - displacement) % 4) + 4) % 4).unwrap()]
            },
            Side::Right => {
                dirs[usize::try_from((((self_index + displacement) % 4) + 4) % 4).unwrap()]
            }
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
                'N' => self.y += amount,
                'S' => self.y -= amount,
                'E' => self.x += amount,
                'W' => self.x -= amount,
                'L' => self.turn(Side::Left, amount),
                'R' => self.turn(Side::Right, amount),
                'F' => {
                    match self.direction {
                        Compass::North => self.y += amount,
                        Compass::South => self.y -= amount,
                        Compass::East => self.x += amount,
                        Compass::West => self.x -= amount,
                    }
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
