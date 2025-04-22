use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let s= buf.next().unwrap().unwrap();

    match s.as_str() {
        "ACE" | "BDF" | "CEG" | "DFA" | "EGB" | "FAC" | "GBD" => println!("Yes"),
        _ => println!("No")
    }
}
