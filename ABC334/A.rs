// https://atcoder.jp/contests/abc334/tasks/abc334_a

use std::io::{stdin, BufRead, self};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let input= buf.next().unwrap().unwrap();
    let v: Vec<usize>= input.trim().split_whitespace().map(|f| f.parse().unwrap()).collect();

    let b= v[0];
    let g= v[1];

    let max_value= b.max(g);

    if max_value == b{
        println!("Bat");
    }else{
        println!("Glove");
    }
}
