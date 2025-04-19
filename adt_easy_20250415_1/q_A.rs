use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap().parse::<u32>().unwrap();

    let mut result= 1;
    for idx in 1..n+1 {
        result *= 2;
    }

    println!("{}", result);
}
