use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

fn play_game(mut deck_1: VecDeque<usize>, mut deck_2: VecDeque<usize>, game: usize) -> Ordering {
    let mut history = HashSet::new();
    let mut round = 1;

    while deck_1.len() > 0
        && deck_2.len() > 0
        && !history.contains(&(deck_1.clone(), deck_2.clone()))
    {
        let player_1_string: Vec<String> = deck_1.iter().map(|x| x.to_string()).collect();
        let player_2_string: Vec<String> = deck_2.iter().map(|x| x.to_string()).collect();

        history.insert((deck_1.clone(), deck_2.clone()));
        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();

        if card_1 <= deck_1.len() && card_2 <= deck_2.len() {
            // Recurse to find the winner
            let new_deck_1 = deck_1
                .iter()
                .take(card_1)
                .map(|x| x.clone())
                .collect::<VecDeque<usize>>();
            let new_deck_2 = deck_2
                .iter()
                .take(card_2)
                .map(|x| x.clone())
                .collect::<VecDeque<usize>>();
            match play_game(new_deck_1, new_deck_2, game + 1) {
                Ordering::Greater => {
                    deck_1.push_back(card_1);
                    deck_1.push_back(card_2);
                }
                Ordering::Less => {
                    deck_2.push_back(card_2);
                    deck_2.push_back(card_1);
                }
                Ordering::Equal => panic!("Decks cannot be equal!"),
            }
        } else {
            // Find the winner the old comparison way
            match card_1.cmp(&card_2) {
                Ordering::Greater => {
                    deck_1.push_back(card_1);
                    deck_1.push_back(card_2);
                }
                Ordering::Less => {
                    deck_2.push_back(card_2);
                    deck_2.push_back(card_1);
                }
                Ordering::Equal => panic!("Cards cannot be equal: {}, {}", card_1, card_2),
            }
        }
        round += 1;
    }

    if history.contains(&(deck_1.clone(), deck_2.clone())) {
        if game == 1 {}
        Ordering::Greater
    } else {
        match deck_1.len().cmp(&deck_2.len()) {
            Ordering::Greater => {
                if game == 1 {
                    println!(
                        "Player 1's winning score: {}",
                        deck_1
                            .iter()
                            .enumerate()
                            .map(|x| (deck_1.len() - x.0) * x.1)
                            .sum::<usize>()
                    );
                }
                Ordering::Greater
            }
            Ordering::Less => {
                if game == 1 {
                    println!(
                        "Player 2's winning score: {}",
                        deck_2
                            .iter()
                            .enumerate()
                            .map(|x| (deck_2.len() - x.0) * x.1)
                            .sum::<usize>()
                    );
                }
                Ordering::Less
            }
            Ordering::Equal => panic!("Decks cannot be same length!"),
        }
    }
}

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

    let score: usize = match play_game(one_deck.clone(), two_deck.clone(), 1) {
        Ordering::Greater => one_deck
            .iter()
            .enumerate()
            .map(|x| (one_deck.len() - x.0) * x.1)
            .sum(),
        Ordering::Less => two_deck
            .iter()
            .enumerate()
            .map(|x| (two_deck.len() - x.0) * x.1)
            .sum(),
        Ordering::Equal => panic!("Decks cannot be equal: {:?}, {:?}", one_deck, two_deck),
    };
}
