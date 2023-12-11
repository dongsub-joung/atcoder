use std::io::{self,BufReader};

fn main(){
    let mut answer= 0;
    let mut v: Vec<usize>= Vec::new();

    let stdin = io::stdin();
    let mut input= stdin.lines();

    let n: usize= input.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..n{
        let number: usize= input.next().unwrap().unwrap().parse().unwrap();
        v.push(number);
    }
    
    let v: Vec<usize>= v.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();
    
    answer= v.len();

    println!("{}", answer);
}