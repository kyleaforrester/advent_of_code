use std::convert::TryFrom;
use std::io::{self, Read};

const Z_DIM_SIZE: usize = 13;

#[derive(Clone)]
enum State {
    Active,
    Inactive,
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let x_dim = buffer.find('\n').unwrap();
    let y_dim = buffer.chars().filter(|&x| x == '\n').count();
    let mut grid: Vec<Vec<Vec<State>>> = Vec::new();
    let input: Vec<Vec<char>> = buffer
        .split('\n')
        .take(y_dim)
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    for _z in 0..Z_DIM_SIZE {
        let mut table = Vec::new();
        for _y in 0..y_dim + 12 {
            table.push(vec![State::Inactive; x_dim + 12]);
        }
        grid.push(table);
    }

    // The middle slice index for Z is 13/2 = 6
    for y in 6..(6 + y_dim) {
        for x in 6..(6 + x_dim) {
            grid[6][y][x] = match input[y - 6][x - 6] {
                '#' => State::Active,
                '.' => State::Inactive,
                _ => panic!("Invalid character"),
            };
        }
    }

    for _i in 0..6 {
        grid = grid_step(grid);
    }

    let mut sum = 0;
    for z_vec in grid {
        for y_vec in z_vec {
            sum += y_vec
                .iter()
                .filter(|s| match s {
                    State::Active => true,
                    State::Inactive => false,
                })
                .count();
        }
    }

    println!("{}", sum);
}

fn grid_step(grid: Vec<Vec<Vec<State>>>) -> Vec<Vec<Vec<State>>> {
    let mut new_grid = grid.clone();

    let z_max = grid.len();
    for z in 0..z_max {
        let y_max = grid[z].len();
        for y in 0..y_max {
            let x_max = grid[z][y].len();
            for x in 0..x_max {
                let active_ns = neighbor_indices(z, z_max - 1, y, y_max - 1, x, x_max - 1)
                    .iter()
                    .map(|n| grid[n[0]][n[1]][n[2]].clone())
                    .filter(|s| match s {
                        State::Active => true,
                        State::Inactive => false,
                    })
                    .count();
                match grid[z][y][x] {
                    State::Active => {
                        if active_ns != 2 && active_ns != 3 {
                            new_grid[z][y][x] = State::Inactive;
                        }
                    }
                    State::Inactive => {
                        if active_ns == 3 {
                            new_grid[z][y][x] = State::Active;
                        }
                    }
                }
            }
        }
    }
    new_grid
}

fn neighbor_indices(
    z: usize,
    z_max: usize,
    y: usize,
    y_max: usize,
    x: usize,
    x_max: usize,
) -> Vec<Vec<usize>> {
    let mut z_vec = Vec::new();
    match usize::try_from(z as isize - 1) {
        Ok(u) => z_vec.push(u),
        Err(_e) => (),
    };
    z_vec.push(z);
    if z + 1 <= z_max {
        z_vec.push(z + 1);
    }

    let mut y_vec = Vec::new();
    match usize::try_from(y as isize - 1) {
        Ok(u) => y_vec.push(u),
        Err(_e) => (),
    };
    y_vec.push(y);
    if y + 1 <= y_max {
        y_vec.push(y + 1);
    }

    let mut x_vec = Vec::new();
    match usize::try_from(x as isize - 1) {
        Ok(u) => x_vec.push(u),
        Err(_e) => (),
    };
    x_vec.push(x);
    if x + 1 <= x_max {
        x_vec.push(x + 1);
    }

    let mut neighbors = neighbor_perms(vec![z_vec, y_vec, x_vec]);
    let self_index = neighbors
        .iter()
        .position(|n| n[0] == z && n[1] == y && n[2] == x)
        .unwrap();
    neighbors.remove(self_index);
    neighbors
}

fn neighbor_perms(vecs: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut perms = Vec::new();
    perms.push(Vec::new());
    for vec in vecs {
        let mut new_perms = Vec::new();
        for perm in perms {
            for e in &vec {
                let mut new_perm = perm.clone();
                new_perm.push(*e);
                new_perms.push(new_perm);
            }
        }
        perms = new_perms;
    }
    perms
}
