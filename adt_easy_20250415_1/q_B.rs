use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap().parse::<usize>().unwrap();

    let mut result_v: Vec<i32>= Vec::new();
    for idx in 0..n {
        let v: Vec<i32>= buf.next().unwrap().unwrap()
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap())
            .collect();

        let sumed: i32= v.iter().sum();

        result_v.push(sumed);
    }

    for idx in 0_usize..n {
        println!("{}", result_v[idx])
    }
}
