use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Eq, PartialEq)]
struct Node {
    name: String,
    neighbors: Vec<String>,
}

impl Node {
    fn new(string: &str) -> Node {
        Node {
            name: string.to_string(),
            neighbors: Vec::new(),
        }
    }
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut nodes: HashMap<String, Node> = HashMap::new();
    for line in buffer.split('\n').filter(|x| x.len() > 0) {
        let rooms = line.split('-').collect::<Vec<&str>>();

        if !nodes.contains_key(rooms[0]) {
            nodes.insert(rooms[0].to_string(), Node::new(rooms[0]));
        }
        if !nodes.contains_key(rooms[1]) {
            nodes.insert(rooms[1].to_string(), Node::new(rooms[1]));
        }

        let room_1 = nodes.get_mut(rooms[0]).unwrap();
        room_1.neighbors.push(rooms[1].to_string());

        let room_2 = nodes.get_mut(rooms[1]).unwrap();
        room_2.neighbors.push(rooms[0].to_string());
    }

    let mut sum = 0;
    let mut stack: Vec<(&Node, HashMap<String, u32>)> = Vec::new();
    stack.push((nodes.get("start").unwrap(), HashMap::new()));
    while stack.len() > 0 {
        let tuple = stack.pop().unwrap();
        let curr_node = tuple.0;
        let mut history = tuple.1;

        let is_minor = curr_node.name.chars().all(|x| x.is_ascii_lowercase());
        if is_minor {
            match history.get_mut(&curr_node.name) {
                Some(h) => *h += 1,
                None => {
                    history.insert(curr_node.name.clone(), 1);
                    {}
                }
            }
        }

        let has_double_room = match history.iter().filter(|x| *x.1 >= 2).nth(0) {
            Some(_h) => true,
            None => false,
        };
        for node_name in curr_node.neighbors.iter() {
            if node_name == "start"
                || (history.contains_key(&node_name.to_string()) && has_double_room)
            {
                continue;
            } else if node_name == "end" {
                sum += 1;
            } else {
                stack.push((nodes.get(node_name).unwrap(), history.clone()));
            }
        }
    }

    println!("{}", sum);
}
