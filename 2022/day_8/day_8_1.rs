use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut grid = Vec::new();
    for line in buffer.split('\n').filter(|x| x.len() > 0) {
        let row: Vec<(u32, bool)> = line
            .chars()
            .map(|x| (x.to_digit(10).unwrap(), false))
            .collect();
        grid.push(row);
    }

    for row in grid.iter_mut() {
        let mut max = 0;
        for tup in row.iter_mut() {
            if tup.0 >= max {
                tup.1 = true;
                max = tup.0 + 1;
            }
        }

        max = 0;
        for tup in row.iter_mut().rev() {
            if tup.0 >= max {
                tup.1 = true;
                max = tup.0 + 1;
            }
        }
    }

    for idx in 0..grid[0].len() {
        let mut max = 0;
        for row in grid.iter_mut() {
            let mut tup = row.get_mut(idx).unwrap();
            if tup.0 >= max {
                tup.1 = true;
                max = tup.0 + 1;
            }
        }

        max = 0;
        for row in grid.iter_mut().rev() {
            let mut tup = row.get_mut(idx).unwrap();
            if tup.0 >= max {
                tup.1 = true;
                max = tup.0 + 1;
            }
        }
    }

    let sum = grid
        .iter()
        .map(|x| x.iter().filter(|y| y.1).count())
        .sum::<usize>();

    println!("{}", sum);
}
