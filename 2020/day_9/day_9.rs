use std::io::{self, Read};

fn is_valid(pre: &[i64], num: i64) -> bool {
    for i in pre.iter().enumerate() {
        for j in &pre[i.0..] {
            if i.1 + j == num && i.1 != j {
                return true;
            }
        }
    }
    false
}

fn find_cont_set_sum(nums: &Vec<i64>, inv: i64) -> Option<i64> {
    for i in 0..nums.len()-3 {
        let mut j = i + 1;
        let mut sum = nums[i] + nums[j];
        while j < nums.len() - 1  && sum < inv {
            j += 1;
            sum += nums[j];
        }

        if sum  == inv {
            let min = &nums[i..j+1].iter().min().unwrap();
            let max = &nums[i..j+1].iter().max().unwrap();
            return Some(**min + **max);
        }
    }

    None
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut items: Vec<i64> = Vec::new();
    for line in buffer.split('\n') {
        let item: i64;
        match line.parse() {
            Ok(n) => item = n,
            Err(_e) => continue,
        }
        items.push(item);
    }

    let invalid = *(items.iter().skip(25).enumerate().filter(|x| !is_valid(&items[x.0..(x.0 + 25)], *x.1)).nth(0).unwrap().1);

    println!("Invalid number: {}", invalid);

    match find_cont_set_sum(&items, invalid) {
        Some(i) => println!("{}", i),
        None => panic!("No contiguous set found!"),
    }
}

