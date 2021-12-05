use std::io::{self, Read};

#[derive(Debug)]
struct Square {
    is_marked: bool,
    value: usize,
}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<Square>>,
    score: Option<usize>,
}

impl Square {
    fn new(val: &str) -> Square {
        Square {
            is_marked: false,
            value: usize::from_str_radix(val.trim(), 10).expect("Not a usize!"),
        }
    }
}

impl Board {
    fn new(string: &str) -> Board {
        let mut grid: Vec<Vec<Square>> = Vec::new();

        for line in string.split('\n') {
            let mut row: Vec<Square> = Vec::new();
            for num in line.split(' ').filter(|x| x.len() > 0) {
                row.push(Square::new(num));
            }
            grid.push(row);
        }

        Board {
            grid: grid,
            score: None,
        }
    }

    fn mark(&mut self, num: usize) {
        // Mark the Square with the called number
        for row in self.grid.iter_mut() {
            let idx: Option<usize> =
                match row.iter().enumerate().filter(|x| x.1.value == num).nth(0) {
                    Some(i) => Some(i.0),
                    None => None,
                };

            match idx {
                Some(i) => row[i].is_marked = true,
                None => (),
            }
        }

        // Now calculate score
        if self.is_winning() {
            self.score = Some(
                self.grid
                    .iter()
                    .map(|x| {
                        x.iter()
                            .filter(|s| !s.is_marked)
                            .map(|s| s.value)
                            .sum::<usize>()
                    })
                    .sum::<usize>()
                    * num,
            );
        } else {
            self.score = None;
        }
    }

    fn is_winning(&self) -> bool {
        // Winning row
        if self
            .grid
            .iter()
            .filter(|r| r.iter().all(|s| s.is_marked))
            .count()
            > 0
        {
            return true;
        }

        // Winning column
        if (0..self.grid[0].len())
            .filter(|i| self.grid.iter().all(|r| r[*i].is_marked))
            .count()
            > 0
        {
            return true;
        }

        false
    }
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let input: Vec<String> = buffer
        .split("\n\n")
        .filter(|x| x.len() > 0)
        .map(|x| x.trim().to_string())
        .collect();

    let drawing: Vec<usize> = input[0]
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    let mut boards: Vec<Board> = Vec::new();

    for board_str in input[1..].iter() {
        boards.push(Board::new(board_str));
    }

    for num in drawing.iter() {
        for board in boards.iter_mut() {
            board.mark(*num);
        }

        if boards.iter().filter(|x| x.score.is_none()).count() == 0 {
            break;
        }

        boards = boards.drain(..).filter(|b| b.score.is_none()).collect();
    }

    let score = boards
        .iter()
        .filter(|x| x.score.is_some())
        .nth(0)
        .unwrap()
        .score
        .unwrap();

    println!("{}", score);
}
