use std::io::{self, BufRead, Stdin, stdin};

fn main() {
    let std= stdin();
    let mut input= std.lock().lines();
    let v: Vec<usize>= input.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();

    let v2:Vec<usize>= input.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();

    let n= v[0];
    let l= v[1];
    let r= v[2];

    for i in 0..n{
        let a= v2[i];
        let mut closeset= &a;

        if closeset < &l{
            closeset= &l;
        }else if closeset> &r{
            closeset= &r;
        }
        println!("{}", closeset);
    }
}

