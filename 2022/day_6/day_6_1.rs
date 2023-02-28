use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

fn all_unique(queue: &VecDeque<char>) -> bool {
    let set: HashSet<&char> = queue.iter().collect();
    return set.len() == 4;
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin!");

    let mut queue: VecDeque<char> = buffer.chars().take(4).collect();
    if all_unique(&queue) {
        println!("4");
        std::process::exit(0);
    }
    for tup in buffer.chars().skip(4).enumerate() {
        queue.pop_front();
        queue.push_back(tup.1);
        if all_unique(&queue) {
            println!("{}", tup.0 + 5);
            std::process::exit(0);
        }
    }
    println!("No start sequence found.");
}
