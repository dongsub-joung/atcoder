use std::io::{self, BufRead, stdin};

fn main() {
    let stdin= stdin();
    let mut input= stdin.lock().lines();

    let inputed_stand_line: Vec<usize>= 
        input.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse().expect("parse err"))
        .collect();
    
    let scores: Vec<usize>= 
        input.next().unwrap().expect("io err")
        .split_whitespace()
        .map(|f| f.parse().expect("parse err"))
        .collect();

    let standard_score= inputed_stand_line[1];
    let mut cnt=0;
    for score in scores{
        if standard_score <= score{
            cnt= cnt+1;
        }
    }

    println!("{}", cnt);
}

