use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();
    let n= buff.next().unwrap().unwrap().parse::<i32>().unwrap();
    let v: Vec<i32>= buff.next().unwrap().unwrap().split_whitespace().map(|f| f.parse::<i32>().unwrap()).collect();
    let x= buff.next().unwrap().unwrap().parse::<i32>().unwrap();
    
    let result= if v.contains(&x) {"Yes"} else {"No"};
    println!("{}", result);
}

