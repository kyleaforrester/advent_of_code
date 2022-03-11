use std::collections::HashSet;
use std::io::{self, Read};

fn find_low_points(rows: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
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
                low_points.push((r_ind, c_ind));
            }
        }
    }
    low_points
}

fn basin_size(low_point: (usize, usize), rows: &Vec<Vec<u32>>) -> usize {
    // Using a stack will iterate the basin in a depth-first style.
    // Use a queue to iterate in a breadth-first style.
    let mut stack = Vec::new();
    let mut visited = HashSet::new();

    stack.push(low_point);

    while stack.len() > 0 {
        let point = stack.pop().unwrap();

        // North
        if point.0 > 0 {
            if rows[point.0 - 1][point.1] < 9 && !visited.contains(&(point.0 - 1, point.1)) {
                stack.push((point.0 - 1, point.1));
            }
        }
        // East
        match rows[point.0].get(point.1 + 1) {
            Some(c) => {
                if *c < 9 && !visited.contains(&(point.0, point.1 + 1)) {
                    stack.push((point.0, point.1 + 1));
                }
            }
            None => (),
        }

        // South
        match rows.get(point.0 + 1) {
            Some(r) => {
                if r[point.1] < 9 && !visited.contains(&(point.0 + 1, point.1)) {
                    stack.push((point.0 + 1, point.1));
                }
            }
            None => (),
        }

        // West
        if point.1 > 0 {
            if rows[point.0][point.1 - 1] < 9 && !visited.contains(&(point.0, point.1 - 1)) {
                stack.push((point.0, point.1 - 1));
            }
        }

        visited.insert(point);
    }

    visited.len()
}

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

    let low_points = find_low_points(&rows);

    let mut basin_sizes: Vec<usize> = low_points.iter().map(|&x| basin_size(x, &rows)).collect();
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));

    println!(
        "{}",
        basin_sizes
            .iter()
            .copied()
            .take(3)
            .reduce(|accum, item| accum * item)
            .unwrap()
    );
}
