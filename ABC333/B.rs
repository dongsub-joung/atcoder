// Is not work

use std::{io::{stdin, BufRead}, fmt::format};

fn main() {
    let mut buf= String::new();
    stdin().read_line(&mut buf).unwrap();
    let input_a= buf.trim();

    let mut buf= String::new();
    stdin().read_line(&mut buf).unwrap();
    let input_b= buf.trim();

    let v=vec!["AE", "AB", "BC", "CD", "DE", "EA", "BA", "CB", "DC", "ED"];
    let v2= vec!["AD","AC", "DA", "CA"];
    let v3= vec!["BE", "BD", "EB", "DB"];
    let v4= vec!["CA", "CE", "AC", "EC"];
    let v5= vec!["DB", "DA", "DA", "BD"];
    let v6= vec!["EB","EC", "BE", "CE"];

    if v.contains(&input_a) && v.contains(&input_b){
        println!("Yes")
    }else{
        if v2.contains(&input_a) && v2.contains(&input_b) {
            println!("Yes")
        }
        if v3.contains(&input_a) && v3.contains(&input_b) {
            println!("Yes")
        }
        if v4.contains(&input_a) && v4.contains(&input_b){
            println!("Yes")
        }
        if v5.contains(&input_a) && v5.contains(&input_b){
            println!("Yes")
        }
        if v6.contains(&input_a) && v6.contains(&input_b){
            println!("Yes")
        }
    }
    println!("No")
}
