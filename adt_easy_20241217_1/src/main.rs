use std::io::{stdin, BufRead};

fn solution_a(){
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap()
    .parse::<u32>().unwrap();
let c= char::from_u32(n).unwrap();

println!("{}", c.to_string());    
}

// https://atcoder.jp/contests/adt_easy_20241217_1/tasks/abc304_a
pub fn solution_b(){
    let mut result_v= Vec::new();
    let mut s_v: Vec<Vec<&str>>= Vec::new();

    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap()
    .parse::<u32>().unwrap();

    for _ in 0..n {
        let person: Vec<&str>= buf.next().unwrap().unwrap().clone()
            .split_whitespace()
            .collect();
        s_v.push(person);
    }
    
    let mut ages: Vec<i32>= Vec::new();

    for v in s_v.clone() {
        ages.push(v[1].parse::<i32>().unwrap());
    }

    let min= ages.iter().min().unwrap();

    let mut idx= 0;
    for (i, v) in s_v.clone().iter().enumerate(){
        let v= v[1].parse::<i32>().unwrap();
        if *min == v{
            idx= i;
        }
    }

    for i in idx..(n as usize){
        result_v.push(s_v[i][1]);
    }

    for i in 0..idx{
        result_v.push(s_v[i][1]);
    }
    
    for v in result_v {
        println!("{}", v);
    }
}

fn main(){
    // https://atcoder.jp/contests/adt_easy_20241217_1/tasks/abc252_a
    // solution_a();

    // https://atcoder.jp/contests/adt_easy_20241217_1/tasks/abc304_a
    solution_b();
}