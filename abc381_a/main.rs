use std::io::{stdin, BufRead};

fn main(){
    // n: u32, s: String
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap()
        .parse::<u32>().unwrap();
    let s= buf.next().unwrap().unwrap();

    if n == 1 && s == "/".to_string(){
        println!("Yes");
    }
    if n % 2 != 0{
        println!("No");
    }

    let mut chars: Vec<char>= s.chars().collect();
    
    for _ in 0..(chars.len() / 2) {
        let poped= chars.pop().unwrap();
        if poped != '2' {
            println!("No");
        }
    }

    println!("{:?}", chars);
    chars.pop();
   
   while let Some(val)= chars.pop() {
        if val != '1' {
            println!("No");
        }
   } 

   println!("Yes");
}
