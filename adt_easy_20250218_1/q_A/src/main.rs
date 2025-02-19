use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();
    let input_v: Vec<i32>= buff.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap() )
        .collect();

    let n= input_v[0];
    let s= input_v[1];
    let k= input_v[2];

    let mut sumed: Vec<i32>= Vec::new();
    for _ in 0..n {
        let input2:Vec<i32>= buff.next().unwrap().unwrap()
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap() )
            .collect();
        sumed.push(input2[0]*input2[1]);
    }

    let mut sougaku: i32= sumed.iter().sum();
    if sougaku < s {
        sougaku += k;
        println!("{}", sougaku);
    }else{
        println!("{}", sougaku);
    }
}
