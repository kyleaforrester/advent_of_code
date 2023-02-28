use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

fn all_unique(queue: &VecDeque<char>) -> bool {
    let set: HashSet<&char> = queue.iter().collect();
    return set.len() == 14;
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin!");

    let mut queue: VecDeque<char> = buffer.chars().take(14).collect();
    if all_unique(&queue) {
        println!("14");
        std::process::exit(0);
    }
    for tup in buffer.chars().skip(14).enumerate() {
        queue.pop_front();
        queue.push_back(tup.1);
        if all_unique(&queue) {
            println!("{}", tup.0 + 15);
            std::process::exit(0);
        }
    }
    println!("No start sequence found.");
}
