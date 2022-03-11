use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut rows: Vec<Vec<u32>> = Vec::new();

    for line in buffer.split('\n').filter(|x| x.len() > 0) {
        let row: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        rows.push(row);
    }

    let mut low_points = Vec::new();
    for r_ind in 0..rows.len() {
        for c_ind in 0..rows[r_ind].len() {
            let curr = rows[r_ind][c_ind];
            let mut neighbors = Vec::new();

            // North
            if r_ind > 0 {
                neighbors.push(rows[r_ind - 1][c_ind]);
            }
            // East
            match rows[r_ind].get(c_ind + 1) {
                Some(c) => neighbors.push(*c),
                None => (),
            }
            // South
            match rows.get(r_ind + 1) {
                Some(r) => neighbors.push(r[c_ind]),
                None => (),
            }
            // West
            if c_ind > 0 {
                neighbors.push(rows[r_ind][c_ind - 1]);
            }

            if neighbors.iter().min().unwrap() > &curr {
                low_points.push(curr);
            }
        }
    }

    println!("{}", low_points.iter().map(|x| x + 1).sum::<u32>());
}
