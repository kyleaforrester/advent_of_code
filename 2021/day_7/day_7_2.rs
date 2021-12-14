use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let nums: Vec<i32> = buffer
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    let optimal_pos = (nums.iter().min().unwrap().clone()..nums.iter().max().unwrap().clone() + 1)
        .map(|x| {
            nums.iter()
                .map(|n| (1..(n - x).abs() + 1).sum::<i32>())
                .sum::<i32>()
        })
        .min()
        .unwrap();

    println!("{}", optimal_pos);
}
