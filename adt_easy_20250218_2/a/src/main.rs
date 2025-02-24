use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let input_v: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap() )
        .collect();

    let x= input_v[0];
    let y= input_v[1];
    let n= input_v[2];

    let three= n / 3;
    let div= n % 3;
    
    if (y / 3) > x {
        println!("{}", x * n);
    }else {
        println!("{}", (y * three) + (div * x));
    }
}