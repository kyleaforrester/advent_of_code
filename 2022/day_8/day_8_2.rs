use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut grid = Vec::new();
    for line in buffer.split('\n').filter(|x| x.len() > 0) {
        let row: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        grid.push(row);
    }

    let mut max = 0;
    // All edge trees will have a product of 0, so skip the edges
    for x in 1..(grid[0].len() - 1) {
        for y in 1..(grid.len() - 1) {
            let height = grid[y][x];

            let mut product = 1;

            // Go south
            let mut y_iter = y;
            let mut count = 0;
            loop {
                if y_iter + 1 >= grid.len() {
                    break;
                }
                if grid[y_iter + 1][x] >= height {
                    count += 1;
                    break;
                }
                count += 1;
                y_iter += 1;
            }
            product *= count;
            count = 0;

            // Go north
            y_iter = y;
            loop {
                if y_iter == 0 {
                    break;
                }
                if grid[y_iter - 1][x] >= height {
                    count += 1;
                    break;
                }
                count += 1;
                y_iter -= 1;
            }
            product *= count;
            count = 0;

            // Go east
            let mut x_iter = x;
            loop {
                if x_iter + 1 >= grid[0].len() {
                    break;
                }
                if grid[y][x_iter + 1] >= height {
                    count += 1;
                    break;
                }
                count += 1;
                x_iter += 1;
            }
            product *= count;
            count = 0;

            // Go west
            x_iter = x;
            loop {
                if x_iter == 0 {
                    break;
                }
                if grid[y][x_iter - 1] >= height {
                    count += 1;
                    break;
                }
                count += 1;
                x_iter -= 1;
            }
            product *= count;

            if product > max {
                max = product;
            }
        }
    }

    println!("{}", max);
}
