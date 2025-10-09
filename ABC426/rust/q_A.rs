use std::io::{stdin, BufRead};



fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();
    let inputed_v: Vec<String>= buff.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.trim().to_string() )
        .collect();
    
    let pivot_idx= get_idx(inputed_v[0].clone());
    let compare_idx= get_idx(inputed_v[1].clone());

    if pivot_idx >= compare_idx {
        println!("Yes");
    }else{
        println!("No");
    }
}

fn get_idx(pivot: String) -> usize{
    let os_version= Vec::from(["Ocelot".to_string(), "Serval".to_string(), "Lynx".to_string()]);

    let mut index= 0;
    for (i, e) in os_version.iter().enumerate() {
        if *e == pivot  {
            index= i;
            return index;
        }   
    }

    index
}

