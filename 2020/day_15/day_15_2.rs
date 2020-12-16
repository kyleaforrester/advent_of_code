use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut nums: Vec<usize> = buffer.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    // Maps a number to the previous index it occurred.
    let mut history: HashMap<usize, usize> = HashMap::new();

    //Populate history
    for n in &nums {
        let max_ind = nums.iter().enumerate().filter(|x| x.1 == n).map(|x| x.0).max().unwrap();
        history.insert(*n, max_ind);
    }

    for i in nums.len()..30000000 {
        let prev = nums[i-1];
        if !history.contains_key(&prev) {
            nums.push(0);
            history.insert(prev, i-1);
        } else {
            let prev_2_ind = history.get(&prev).unwrap();
            nums.push((i - 1) - prev_2_ind);
            history.insert(prev, i - 1);
        }
    }

    println!("{}", nums[29999999]);
}

