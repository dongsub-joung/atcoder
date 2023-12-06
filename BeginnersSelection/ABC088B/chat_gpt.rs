use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let _n: usize = input.next().unwrap().expect("Couldn't read line")
        .trim().parse().expect("Parsing error");

    let mut cards: Vec<i32> = input.next().unwrap().expect("Couldn't read line")
        .split_whitespace()
        .map(|s| s.parse().expect("Parsing error"))
        .collect();

    cards.sort();
    cards.reverse();

    let mut alice_score = 0;
    let mut bob_score = 0;

    for (index, &card) in cards.iter().enumerate() {
        if index % 2 == 0 {
            alice_score += card;
        } else {
            bob_score += card;
        }
    }

    let difference = alice_score - bob_score;
    println!("{}", difference);
}

