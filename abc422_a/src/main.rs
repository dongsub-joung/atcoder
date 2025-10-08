use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let input_str= buf.next().unwrap().unwrap();
    let mut stage= input_str.as_bytes()[0];
    let mut round= input_str.as_bytes()[2];
    let gep= round+1;
    if gep >= 8 {
        stage+=1;
        round=0;
        round+= (8 - gep);
    }else{
        round+=1
    }

    println!("{}-{}", stage as char,round as char);
}
