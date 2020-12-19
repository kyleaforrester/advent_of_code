use std::convert::TryFrom;
use std::io::{self, Read};

const Z_DIM_SIZE: usize = 13;
const W_DIM_SIZE: usize = 13;

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
    let mut grid: Vec<Vec<Vec<Vec<State>>>> = Vec::new();
    let input: Vec<Vec<char>> = buffer
        .split('\n')
        .take(y_dim)
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    for _w in 0..W_DIM_SIZE {
        let mut z_grid = Vec::new();
        for _z in 0..Z_DIM_SIZE {
            let mut table = Vec::new();
            for _y in 0..y_dim + 12 {
                table.push(vec![State::Inactive; x_dim + 12]);
            }
            z_grid.push(table);
        }
        grid.push(z_grid);
    }

    // The middle slice index for Z is 13/2 = 6
    for y in 6..(6 + y_dim) {
        for x in 6..(6 + x_dim) {
            grid[6][6][y][x] = match input[y - 6][x - 6] {
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
    for w_vec in grid {
        for z_vec in w_vec {
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
    }

    println!("{}", sum);
}

fn grid_step(grid: Vec<Vec<Vec<Vec<State>>>>) -> Vec<Vec<Vec<Vec<State>>>> {
    let mut new_grid = grid.clone();

    let w_max = grid.len();
    for w in 0..w_max {
        let z_max = grid[w].len();
        for z in 0..z_max {
            let y_max = grid[w][z].len();
            for y in 0..y_max {
                let x_max = grid[w][z][y].len();
                for x in 0..x_max {
                    let active_ns =
                        neighbor_indices(w, w_max - 1, z, z_max - 1, y, y_max - 1, x, x_max - 1)
                            .iter()
                            .map(|n| grid[n[0]][n[1]][n[2]][n[3]].clone())
                            .filter(|s| match s {
                                State::Active => true,
                                State::Inactive => false,
                            })
                            .count();
                    match grid[w][z][y][x] {
                        State::Active => {
                            if active_ns != 2 && active_ns != 3 {
                                new_grid[w][z][y][x] = State::Inactive;
                            }
                        }
                        State::Inactive => {
                            if active_ns == 3 {
                                new_grid[w][z][y][x] = State::Active;
                            }
                        }
                    }
                }
            }
        }
    }
    new_grid
}

fn neighbor_indices(
    w: usize,
    w_max: usize,
    z: usize,
    z_max: usize,
    y: usize,
    y_max: usize,
    x: usize,
    x_max: usize,
) -> Vec<Vec<usize>> {
    let mut w_vec = Vec::new();
    match usize::try_from(w as isize - 1) {
        Ok(u) => w_vec.push(u),
        Err(_e) => (),
    };
    w_vec.push(w);
    if w + 1 <= w_max {
        w_vec.push(w + 1);
    }

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

    let mut neighbors = neighbor_perms(vec![w_vec, z_vec, y_vec, x_vec]);
    let self_index = neighbors
        .iter()
        .position(|n| n[0] == w && n[1] == z && n[2] == y && n[3] == x)
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
