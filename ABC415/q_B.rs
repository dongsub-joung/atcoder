use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let s= std.lock().lines().next().unwrap().unwrap();
    let v: Vec<&str>= s.trim().split("").collect();

    let mut indexs: Vec<usize>= Vec::new();
    for (i, e) in v.iter().enumerate() {
        if e == &"#" {
            indexs.push(i);
        }
    }

    let mut i= 0;
    let mut j= 1;
    while j < indexs.len() {
        let a= indexs[i];
        let b= indexs[j];

        println!("{},{}", a,b);
        i+=2;
        j+=2;
    }
}

