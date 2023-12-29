use std::io::{stdin, BufRead};

fn main() {
    let mut count= 0;

    let std= stdin();
    let mut buf= std.lock().lines();
    let input= buf.next().unwrap().unwrap();
    let v: Vec<i128>= input.trim().split_whitespace().map(|f| f.parse().unwrap()).collect();

    let a= v[0];
    let m= v[1];
    let l= v[2];
    let r= v[3];

    let mut v2: Vec<i128>= Vec::new();
    let mut foward=a; 
    let mut back= a-m;
    loop {
        v2.push(foward);
        if foward > r {
            break;
        }
        foward+=m;
    }

    loop {
        v2.push(back);
        if back < l {
            break;
        }
        back-=m;
    }

    v2.sort();
    for element in v{
        if l<= element && element <= r{
            count+=1;
        }
    }

    println!("{}", count);
}
