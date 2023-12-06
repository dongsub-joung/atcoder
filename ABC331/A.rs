use std::io::{self, BufRead, stdin};

fn main() {
    let stdin= stdin();
    let mut input= stdin.lock().lines();

    let limite: Vec<usize>= input.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();
    let day: Vec<usize>= input.next().unwrap().unwrap().split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();

    let M= limite[0];
    let D= limite[1];

    let mut yyyy= day[0];
    let mut mm= day[1];
    let mut dd= day[2];

    dd= dd+1;

    if dd > D{
        mm= mm+1;
        dd= 1;
    }

    if mm > M{
        yyyy= yyyy+1;
        mm= 1;
    }

    println!("{} {} {}", yyyy, mm, dd)
}  
