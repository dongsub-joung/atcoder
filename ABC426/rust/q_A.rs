use std::io::{stdin, BufRead};



fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();
    let inputed_v: Vec<String>= buff.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.trim().to_string() )
        .collect();
    
    get_idx(inputed_v[0].clone(), inputed_v[1].clone());
    
}

fn get_idx(pivot: String, with_os: String) -> bool{
    let os_version= Vec::from(["Ocelot".to_string(), "Serval".to_string(), "Lynx".to_string()]);
    
    let mut indexs: Vec<usize>= Vec::new();
    for (i, e) in os_version.iter().enumerate() {
        for j in  {
            
        }
        if *e == pivot  {
            indexs.push(i);
        }
    }

    for e in indexs.iter().next() {
        println!("{}", e);
    }

    false
}

