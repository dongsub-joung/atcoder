use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let input1: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap())
        .collect();

    let input2: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap())
        .collect();

    let target_number= input1[1];
    let mut sum=0_i32;
    let numbers_v= input2;
    for e in numbers_v{ 
        if e <= target_number {
            sum+= e;
        }
    }

    println!("{}", sum);
}
