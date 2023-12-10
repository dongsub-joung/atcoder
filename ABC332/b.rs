use std::io::{self, BufRead,stdin};

fn main() {
    let mut answer= 0;
    
    let std= stdin();
    let mut input= std.lock().lines();
    let v:Vec<usize>= input.next().unwrap().unwrap().split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();

    let k= v[0];
    let g= v[1];
    let m= v[2];

    let mut glass= 0;
    let mut magu= 0;

    for _ in 0..k{
        if glass == g{
            glass=0;
        }else if magu == 0{
            magu= m;
        }else{
            let drop= magu.min(g-glass);
            magu= magu-drop;
            glass= glass+drop;
        }
    }

    println!("{} {}", glass, magu);
}

