use std::io::{self, BufRead,stdin};

fn main() {
    let std= stdin();
    let mut input= std.lock().lines();
    let mut vectors: Vec<Vec<usize>>= Vec::new();
    let v:Vec<usize>= input.next().unwrap().unwrap().split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();


    let n= v[0];
    let s= v[1];
    let k= v[2];
    let mut answer= 0;

    for _ in 0..n {
        let product: Vec<usize>= input.next().unwrap().unwrap().split_whitespace()
            .map(|f| f.parse().unwrap())
            .collect();
        vectors.push(product);         
    }

    let mut cals: Vec<usize>= Vec::new();
    for v in vectors{
        let p= v[0];
        let q= v[1];
        cals.push(p * q);
    }

    let sum: usize= cals.iter().sum();
    if  sum >= s{
        answer= sum;
    }else{
        answer= sum+k;
    }

    println!("{}", answer);
}

