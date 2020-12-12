use std::io::{self, Read};
use std::convert::TryFrom;

#[derive(Clone, Copy)]
enum SeatState {
    Floor,
    Empty,
    Taken,
}

struct Ferry {
    seats: Vec<Vec<SeatState>>
}

impl Ferry {
    fn new(s: &str) -> Ferry {
        let mut seats = Vec::new();
        for line in s.split('\n') {
            if line.len() == 0 {
                continue;
            }
            let row: Vec<SeatState> = line.chars().map(|x| match x { '.' => SeatState::Floor, 'L' => SeatState::Empty, '#' => SeatState::Taken, _ => panic!(format!("Not a valid seat: {}", x))}).collect();
            seats.push(row);
        }
        
        Ferry {
            seats: seats,
        }
    }

    fn iterate(&mut self) {
        let directions = [(-1,-1), (0,-1), (1,-1), (-1,0), (1,0), (-1,1), (0,1), (1,1)];
        let max_y = self.seats.len();
        let mut new_seats = Vec::new();
        for row in self.seats.iter().enumerate() {
            let max_x = row.1.len();
            let mut new_row = Vec::new();
            for seat in row.1.iter().enumerate() {
                let new_seat = match seat.1 {
                    SeatState::Floor => SeatState::Floor,
                    _ => {
                        let mut sum = 0;
                        for d in directions.iter() {
                            let x = i32::try_from(seat.0).unwrap() + d.0;
                            let y = i32::try_from(row.0).unwrap() + d.1;
                            let x = match usize::try_from(x) {
                                Ok(u) => u,
                                Err(_e) => continue,
                            };
                            let y = match usize::try_from(y) {
                                Ok(u) => u,
                                Err(_e) => continue,
                            };

                            if x < max_x && y < max_y {
                                sum += match self.seats[y][x] {
                                    SeatState::Floor => 0,
                                    SeatState::Empty => 0,
                                    SeatState::Taken => 1,
                                };
                            }
                        }
                        if sum == 0 {
                            SeatState::Taken
                        } else if sum >= 4 {
                            SeatState::Empty
                        } else {
                            (*seat.1).clone()
                        }
                    },
                };
                new_row.push(new_seat);
            }
            new_seats.push(new_row);
        }

        self.seats = new_seats;
    }

    fn occupied_count(&self) -> i32 {
        let mut sum = 0;
        for row in &self.seats {
            for seat in row {
                sum += match seat {
                    SeatState::Floor => 0,
                    SeatState::Empty => 0,
                    SeatState::Taken => 1,
                };
            }
        }
        sum
    }
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut ferry = Ferry::new(&buffer);

    let mut old_occ_count = ferry.occupied_count();
    loop {
        ferry.iterate();
        let new_occ_count = ferry.occupied_count();
        if old_occ_count == new_occ_count {
            break;
        }
        old_occ_count = new_occ_count;
    }

    println!("{}", old_occ_count);
}

