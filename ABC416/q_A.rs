use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();

    let v:Vec<usize>= buff
        .next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<usize>().unwrap() )
        .collect();
    let s= buff
        .next().unwrap().unwrap();
    
    let mut bool_v:Vec<bool>= Vec::new();
    let chars= s.as_bytes();
    
    let mut idx= v[1];
    let mut j= 1_usize;
    while j < v[2]+1{
        if j >= idx{
            if chars[j-1] == 111 {
                bool_v.push(true);    
            }else{
                bool_v.push(false);
            }   
        }
        j+=1;
    }

    let mut cnt= 0_usize;
    let expect= v[2] - v[1];

    let mut flag= true;
    for e in bool_v {
        if e == false {
            flag= false;
        }
    }

    if flag {
        println!("Yes");
    }else{
        println!("No");
    }
}

