use std::io::{stdin, BufRead};

pub fn main(){
    let std= stdin();
    let mut buf= std.lock().lines();
    let n: usize= buf.next().unwrap().unwrap().parse().unwrap();

    let mut str_result= String::new();
    for i in 0..n*2+1{
        let j= i.clone() % 2;
        match j {
            0_usize => str_result.push_str("1"),
            1_usize => str_result.push_str("0"),
            _ => println!("Unvalid")
        }
    }
    
    println!("{}", str_result);
}

