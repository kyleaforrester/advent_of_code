use std::convert::TryInto;
use std::io::{self, Read};

const STEPS: u32 = 100;

enum OctState {
    Stale,
    Charged,
    Fired,
}

struct Oct {
    state: OctState,
    charge: u32,
}

impl Oct {
    fn new(string: &str) -> Oct {
        Oct {
            state: OctState::Stale,
            charge: string.parse::<u32>().unwrap(),
        }
    }

    fn flash(board: &mut Vec<Vec<Oct>>, tup_parm: (usize, usize)) {
        let mut stack = Vec::new();

        stack.push(tup_parm);

        while stack.len() > 0 {
            let tup = stack.pop().unwrap();

            let mut oct = &mut board[tup.0][tup.1];
            oct.state = OctState::Fired;

            let mut row_adj = -1;
            while row_adj <= 1 {
                let mut col_adj = -1;
                while col_adj <= 1 {
                    let new_row = row_adj + (tup.0 as i32);
                    let new_col = col_adj + (tup.1 as i32);
                    if new_row >= 0
                        && new_row < board.len().try_into().unwrap()
                        && new_col >= 0
                        && new_col < board[new_row as usize].len().try_into().unwrap()
                    {
                        let row = new_row as usize;
                        let col = new_col as usize;
                        match board[row][col].state {
                            OctState::Stale => {
                                board[row][col].charge += 1;
                                if board[row][col].charge == 10 {
                                    board[row][col].state = OctState::Charged;
                                    stack.push((row, col));
                                }
                            }
                            OctState::Charged => (),
                            OctState::Fired => (),
                        }
                    }
                    col_adj += 1;
                }
                row_adj += 1;
            }
        }
    }
}

fn print_board(board: &Vec<Vec<Oct>>) {
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            print!("{}", board[row][col].charge);
        }
        println!();
    }
    println!();
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut board: Vec<Vec<Oct>> = Vec::new();
    for line in buffer.split('\n').filter(|x| x.len() > 0) {
        board.push(line.chars().map(|x| Oct::new(&x.to_string())).collect());
    }

    let mut sum = 0;
    for _i in 0..STEPS {
        for row in 0..board.len() {
            for col in 0..board[row].len() {
                board[row][col].charge += 1;
                if board[row][col].charge == 10 {
                    board[row][col].state = OctState::Charged;
                }
            }
        }

        for row in 0..board.len() {
            for col in 0..board[row].len() {
                match board[row][col].state {
                    OctState::Stale => (),
                    OctState::Charged => Oct::flash(&mut board, (row, col)),
                    OctState::Fired => (),
                }
            }
        }

        for row in 0..board.len() {
            for col in 0..board[row].len() {
                match board[row][col].state {
                    OctState::Stale => (),
                    OctState::Charged => panic!("Octopus cannot be charged after the firings!"),
                    OctState::Fired => {
                        sum += 1;
                        board[row][col].charge = 0;
                        board[row][col].state = OctState::Stale;
                    }
                }
            }
        }
    }

    println!("{}", sum);
}
