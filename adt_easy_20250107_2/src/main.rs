use std::io::{stdin, BufRead};

fn solution_a(){
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap().parse::<i32>().unwrap_or(
        -9999
    );
    
    if n == - 9999{
        println!("No");
    }else{
        println!("Yes");
    }
}

fn main() {
    solution_a();
}
