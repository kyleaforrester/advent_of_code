use std::collections::HashSet;
use std::io::{self, Read};

fn print_points(points: &HashSet<(usize, usize)>) {
    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();

    println!("{}:", points.len());
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin.");

    let parts: Vec<&str> = buffer.split("\n\n").collect();

    let mut points = HashSet::new();
    for line in parts[0].split('\n').filter(|x| x.len() > 0) {
        let dimensions: Vec<usize> = line
            .split(',')
            .map(|x| x.parse::<usize>().expect("Not an integer!"))
            .collect();
        points.insert((dimensions[0], dimensions[1]));
    }

    let mut folds = Vec::new();
    for line in parts[1]
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| x.strip_prefix("fold along ").unwrap())
    {
        let fold_parsed: Vec<&str> = line.split('=').collect();
        folds.push((
            fold_parsed[0].parse::<char>().expect("Not a char!"),
            fold_parsed[1].parse::<usize>().expect("Not an integer!"),
        ));
    }

    print_points(&points);

    for f in folds.iter().take(1) {
        let mut new_points = HashSet::new();
        if f.0 == 'x' {
            for p in points.iter() {
                if p.0 < f.1 {
                    new_points.insert((p.0, p.1));
                } else if p.0 > f.1 {
                    new_points.insert((2 * f.1 - p.0, p.1));
                }
            }
        } else {
            for p in points.iter() {
                if p.1 < f.1 {
                    new_points.insert((p.0, p.1));
                } else if p.1 > f.1 {
                    new_points.insert((p.0, 2 * f.1 - p.1));
                }
            }
        }
        points = new_points;
        print_points(&points);
    }

    println!("{}", points.len());
}
