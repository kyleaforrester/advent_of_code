use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Read};
use std::rc::{Rc, Weak};

struct File {
    name: String,
    size: usize,
}

struct Dir {
    name: String,
    parent: Weak<RefCell<Dir>>,
    children: Vec<Rc<RefCell<Dir>>>,
    files: Vec<File>,
}

fn absolute_name(dir: &Rc<RefCell<Dir>>) -> String {
    let mut names = Vec::new();
    let mut parent;
    let mut curr_dir = Rc::clone(&dir);
    while curr_dir.borrow().parent.strong_count() > 0 {
        names.push(curr_dir.borrow().name.to_string());
        parent = Rc::clone(&curr_dir.borrow().parent.upgrade().unwrap());
        curr_dir = Rc::clone(&parent);
    }
    if names.len() == 0 {
        names.push(String::from("/"));
    } else {
        names.push(String::from(""));
    }

    names.into_iter().rev().collect::<Vec<String>>().join("/")
}

fn sum_nodes(dir: &Rc<RefCell<Dir>>, size_map: &mut HashMap<String, usize>) -> usize {
    let file_sum: usize = dir.borrow().files.iter().map(|x| x.size).sum();
    let child_sum: usize = dir
        .borrow()
        .children
        .iter()
        .map(|x| sum_nodes(&x, size_map))
        .sum();
    let sum = file_sum + child_sum;

    size_map.insert(absolute_name(dir), sum);
    sum
}

fn ls(curr_dir: &Rc<RefCell<Dir>>, command: &str) {
    for output_line in command
        .split('\n')
        .skip(1)
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
    {
        let line_parts: Vec<&str> = output_line.split(' ').collect();
        match line_parts[0] {
            "dir" => {
                let child_absent = curr_dir
                    .borrow()
                    .children
                    .iter()
                    .find(|&x| x.borrow().name == line_parts[1])
                    .is_none();
                if child_absent {
                    (*curr_dir.borrow_mut())
                        .children
                        .push(Rc::new(RefCell::new(Dir {
                            name: String::from(line_parts[1]),
                            parent: Rc::downgrade(curr_dir),
                            children: Vec::new(),
                            files: Vec::new(),
                        })));
                }
            }
            _ => {
                let file_absent = curr_dir
                    .borrow()
                    .files
                    .iter()
                    .find(|&x| x.name == line_parts[1])
                    .is_none();
                if file_absent {
                    (*curr_dir.borrow_mut()).files.push(File {
                        name: String::from(line_parts[1]),
                        size: line_parts[0].parse().unwrap(),
                    });
                }
            }
        }
    }
}

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let root = Rc::new(RefCell::new(Dir {
        name: String::from("/"),
        parent: Weak::new(),
        children: Vec::new(),
        files: Vec::new(),
    }));
    let mut curr_dir = Rc::clone(&root);
    for command in buffer.split('$').filter(|x| x.len() > 0) {
        let input: Vec<&str> = command
            .split('\n')
            .nth(0)
            .unwrap()
            .trim()
            .split(' ')
            .collect();
        match input[0] {
            "ls" => ls(&mut curr_dir, command),
            "cd" => match input[1] {
                "/" => curr_dir = Rc::clone(&root),
                ".." => {
                    let parent = Rc::clone(&curr_dir.borrow().parent.upgrade().unwrap());
                    curr_dir = Rc::clone(&parent);
                }
                _ => {
                    let child = Rc::clone(
                        &curr_dir
                            .borrow()
                            .children
                            .iter()
                            .find(|&x| x.borrow().name == input[1])
                            .unwrap(),
                    );
                    curr_dir = Rc::clone(&child);
                }
            },
            _ => panic!("Undefined shell command: {}", input[0]),
        }
    }

    let mut size_map = HashMap::new();
    sum_nodes(&root, &mut size_map);

    let total_size = size_map.iter().max_by_key(|x| x.1).unwrap().1;
    let needed_space = *total_size as u64 - 40000000u64;
    println!(
        "{}",
        size_map
            .iter()
            .filter(|x| *x.1 as u64 >= needed_space)
            .min_by_key(|x| x.1)
            .unwrap()
            .1
    );
}
