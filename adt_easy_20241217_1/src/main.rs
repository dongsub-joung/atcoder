use std::io::{stdin, BufRead};

fn solution_a(){
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap()
    .parse::<u32>().unwrap();
let c= char::from_u32(n).unwrap();

println!("{}", c.to_string());    
}

fn main(){
    // https://atcoder.jp/contests/adt_easy_20241217_1/tasks/abc252_a
    solution_a();

    // https://atcoder.jp/contests/adt_easy_20241217_1/tasks/abc304_a
    
}