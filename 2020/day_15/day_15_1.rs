use std::io::{self, Read};
use std::convert::TryFrom;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut nums: Vec<usize> = buffer.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect();

    for i in nums.len()..2020 {
        let prev = nums[i-1];
        if nums.iter().filter(|&&x| x == prev).count() == 1 {
            nums.push(0);
        } else {
            let last_two: Vec<(usize, usize)> = nums.iter().enumerate().rev().filter(|x| *(x.1) == prev).map(|x| (x.0, *x.1)).collect();
            nums.push(usize::try_from(last_two[0].0 - last_two[1].0).unwrap());
        }
    }

    println!("{}", nums[2019]);
}

