use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut one_deck: VecDeque<usize> = VecDeque::new();
    let mut two_deck: VecDeque<usize> = VecDeque::new();
    let mut deck = &mut one_deck;
    for player_string in buffer.split("\n\n").filter(|x| x.len() > 0) {
        for line in player_string.split('\n').filter(|x| x.len() > 0) {
            match line {
                "Player 1:" => deck = &mut one_deck,
                "Player 2:" => deck = &mut two_deck,
                _ => deck.push_back(line.parse::<usize>().unwrap()),
            }
        }
    }

    while one_deck.len() > 0 && two_deck.len() > 0 {
        if one_deck[0] > two_deck[0] {
            let card = one_deck.pop_front().unwrap();
            one_deck.push_back(card);
            one_deck.push_back(two_deck.pop_front().unwrap());
        } else {
            let card = two_deck.pop_front().unwrap();
            two_deck.push_back(card);
            two_deck.push_back(one_deck.pop_front().unwrap());
        }
    }

    let score: usize = if one_deck.len() == 0 {
        two_deck
            .iter()
            .enumerate()
            .map(|x| (two_deck.len() - x.0) * x.1)
            .sum()
    } else {
        one_deck
            .iter()
            .enumerate()
            .map(|x| (one_deck.len() - x.0) * x.1)
            .sum()
    };

    println!("{}", score);
}
