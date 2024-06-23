use std::{io::{stdin, BufRead}, result};

fn main() {
    // input 1
    let std= stdin();
    let mut buff= std.lock().lines();
    let input1: u32= buff.next().unwrap().unwrap().parse().unwrap();
    
    // input 2
    let mut input2_v: Vec<u32>= buff.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|f| f.parse().unwrap() )
        .collect();

    // allocation
    let (input2, input3)= (input2_v[0], input2_v[1]);

    // input 3
    let mut name= buff.next().unwrap().unwrap().trim()
        .to_string();

    // progress
    let sumed= input1 + input2 + input3;
    println!("{} {}", sumed, name);
}

