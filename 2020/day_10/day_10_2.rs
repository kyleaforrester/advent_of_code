use std::io::{self, Read};
use std::convert::TryFrom;
use std::collections::VecDeque;

struct Adapter {
    jolts: i32,
    is_backbone: bool,
}

impl Adapter {
    fn new(s: &str) -> Result<Adapter, String> {
        let jolts = match s.parse::<i32>() {
            Ok(i) => i,
            Err(_e) => return Err(format!("Not an integer: {}", s)),
        };

        Ok(Adapter {
            jolts: jolts,
            is_backbone: false,
        })
    }
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).expect("Could not read from stdin");

    let mut adapters = Vec::new();
    for line in buffer.split('\n') {
        let adapter = match Adapter::new(line) {
            Ok(a) => a,
            Err(_e) => continue,
        };
        adapters.push(adapter);
    }

    adapters.sort_by(|a, b| a.jolts.cmp(&b.jolts));

    let max_jolts = adapters[adapters.len()-1].jolts;
    adapters.insert(0, Adapter {
        jolts: 0,
        is_backbone: true,
    });
    adapters.push(
        Adapter {
            jolts: max_jolts + 3,
            is_backbone: true,
        }
    );

    label_backbone(&mut adapters);

    let backbone_inds: Vec<usize> = adapters.iter().enumerate().filter(|x| x.1.is_backbone).map(|x| x.0).collect();

    let mut group_perms = Vec::new();
    for i in backbone_inds.iter().zip(backbone_inds.iter().skip(1)) {
        group_perms.push(group_permutations(&adapters[*i.0..*i.1+1]));
    }

    println!("{}", group_perms.iter().fold(1u64, |acc, x| acc * u64::try_from(*x).unwrap()));
}

fn label_backbone(adapters: &mut Vec<Adapter>) {
    for i in 0..adapters.len()-1 {
        if adapters[i+1].jolts - adapters[i].jolts == 3 {
            adapters[i].is_backbone = true;
            adapters[i+1].is_backbone = true;
        }
    }
}

fn group_permutations(adapters: &[Adapter]) -> i32 {
    let min = adapters[0].jolts;
    let max = adapters[adapters.len()-1].jolts;
    // Turn into i32 list, and strip first and last element (backbones)
    let jolts: Vec<i32> = adapters.iter().map(|x| x.jolts).take(adapters.len()-1).skip(1).collect();

    let mut sum = 0;
    for mut c in combinations(&jolts) {
        c.insert(0, min);
        c.push(max);
        if is_valid_circuit(c) {
            sum += 1;
        }
    }
    sum
}

fn combinations(jolts: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut circuits = Vec::new();
    let mut queue = VecDeque::new();

    // The empty vector will be missed by below loop so adding here
    circuits.push(Vec::<i32>::new());

    queue.push_back((Vec::<i32>::new(), jolts.clone()));
    while queue.len() > 0 {
        let tup = queue.pop_front().unwrap();
        for i in 0..tup.1.len() {
            let mut new_target = tup.0.clone();
            let new_data = tup.1.iter().skip(i+1).map(|x| *x).collect();
            new_target.push(tup.1[i]);
            circuits.push(new_target.clone());
            queue.push_back((new_target, new_data));
        }
    }
    circuits
}

fn is_valid_circuit(c: Vec<i32>) -> bool {
    for duo in c.iter().zip(c.iter().skip(1)) {
        if duo.1 - duo.0 > 3 {
            return false;
        }
    }
    true
}
