// 入力例 3
// index out of bounds: the len is 2 but the index is 2

use std::io::{stdin, BufRead};

fn main(){
    let std= stdin();
    let mut buff= std.lock().lines();
    let hw: Vec<usize>= buff.next().unwrap().unwrap()
        .split_whitespace().map(|f| f.parse().unwrap()).collect();
    let h= hw[0];
    let w= hw[1];

    let mut a_vecs:Vec<Vec<usize>>= Vec::new();
    for _ in 0..h {
        let a: Vec<usize>= buff.next().unwrap().unwrap()
            .split_whitespace().map(|f| f.parse().unwrap()).collect();
        a_vecs.push(a);
    }

    let mut h_sum:Vec<usize>= Vec::new();
    let mut w_sum:Vec<usize>= Vec::new();
    for _ in 0..h{
        h_sum.push(0);
        w_sum.push(0);
    }

    for i in 0..h{
        for j in 0..w{
            h_sum[i]+= a_vecs[i][j];
            w_sum[j]+= a_vecs[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w{
            print!("{} ", h_sum[i] + w_sum[j] - a_vecs[i][j]);
        }
        println!()
    }
}