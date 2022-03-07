use std::collections::HashMap;
use std::io::{self, Read};

fn hce_calculate_map(string: &str) -> HashMap<char, char> {
    let mut map: HashMap<char, char> = HashMap::new();
    let examples: Vec<&str> = string
        .split(" | ")
        .nth(0)
        .unwrap()
        .trim()
        .split(' ')
        .collect();

    let seven = examples.iter().filter(|x| x.len() == 3).nth(0).unwrap();
    let one = examples.iter().filter(|x| x.len() == 2).nth(0).unwrap();
    let four = examples.iter().filter(|x| x.len() == 4).nth(0).unwrap();
    let nine = examples
        .iter()
        .filter(|x| x.len() == 6 && four.chars().all(|y| x.contains(y)))
        .nth(0)
        .unwrap();
    let six = examples
        .iter()
        .filter(|x| x.len() == 6 && !seven.chars().all(|y| x.contains(y)))
        .nth(0)
        .unwrap();
    let three = examples
        .iter()
        .filter(|x| x.len() == 5 && seven.chars().all(|y| x.contains(y)))
        .nth(0)
        .unwrap();
    let five = examples
        .iter()
        .filter(|x| x.len() == 5 && x.chars().all(|y| six.contains(y)))
        .nth(0)
        .unwrap();
    let zero = examples
        .iter()
        .filter(|x| x.len() == 6 && *x != nine && *x != six)
        .nth(0)
        .unwrap();

    map.insert(
        seven.chars().filter(|&x| !one.contains(x)).nth(0).unwrap(),
        'a',
    );
    map.insert(
        four.chars().filter(|&x| !six.contains(x)).nth(0).unwrap(),
        'c',
    );
    map.insert(
        one.chars()
            .filter(|y| y != map.iter().filter(|x| *x.1 == 'c').nth(0).unwrap().0)
            .nth(0)
            .unwrap(),
        'f',
    );
    map.insert(
        six.chars().filter(|&x| !five.contains(x)).nth(0).unwrap(),
        'e',
    );
    map.insert(
        nine.chars().filter(|&x| !zero.contains(x)).nth(0).unwrap(),
        'd',
    );
    map.insert(
        four.chars().filter(|&x| !three.contains(x)).nth(0).unwrap(),
        'b',
    );
    map.insert(
        three
            .chars()
            .filter(|&x| !four.contains(x) && !seven.contains(x))
            .nth(0)
            .unwrap(),
        'g',
    );

    map
}

fn map_to_number(string: &str, map: &HashMap<char, char>) -> u32 {
    let chars: Vec<char> = string
        .chars()
        .map(|x| map.get(&x).unwrap().clone())
        .collect();

    match chars.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => {
            if ['a', 'c', 'd', 'e', 'g'].iter().all(|x| chars.contains(x)) {
                2
            } else if ['a', 'c', 'd', 'f', 'g'].iter().all(|x| chars.contains(x)) {
                3
            } else if ['a', 'b', 'd', 'f', 'g'].iter().all(|x| chars.contains(x)) {
                5
            } else {
                panic!("This 5 char number is wrong: {:?}", chars);
            }
        }
        6 => {
            if ['a', 'b', 'c', 'e', 'f', 'g']
                .iter()
                .all(|x| chars.contains(x))
            {
                0
            } else if ['a', 'b', 'd', 'e', 'f', 'g']
                .iter()
                .all(|x| chars.contains(x))
            {
                6
            } else if ['a', 'b', 'c', 'd', 'f', 'g']
                .iter()
                .all(|x| chars.contains(x))
            {
                9
            } else {
                panic!("This 6 char number is wrong: {:?}", chars);
            }
        }
        7 => 8,
        _ => panic!("Invalid number char length: {}", chars.len()),
    }
}

fn calculate_output(string: &str) -> u32 {
    //The map will keep track of possible numbers each character could belong to
    let map = hce_calculate_map(string);

    string
        .split(" | ")
        .nth(1)
        .unwrap()
        .split(' ')
        .enumerate()
        .map(|x| 10u32.pow(3u32 - (x.0 as u32)) * map_to_number(x.1, &map))
        .sum()
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut values = Vec::new();
    for line in buffer.split('\n').filter(|x| x.len() > 0) {
        values.push(calculate_output(line));
    }

    println!("{}", values.iter().sum::<u32>());
}
