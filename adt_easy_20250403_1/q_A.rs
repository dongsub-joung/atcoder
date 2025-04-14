use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap().parse::<usize>().unwrap();
    let s_v= buf.next().unwrap().unwrap();
    let bytes= s_v.as_bytes();

    let mut bool_v: Vec<bool>= Vec::new();
    for i in 1_usize..n{
        let pre_c= bytes[i-1] as char;
        let c= bytes[i] as char;
        if pre_c == c{
            bool_v.push(false);
        }else {
            bool_v.push(true);
        }
    }

    if n == 1{
        println!("Yes");
    }else if bool_v.contains(&false) {
        println!("No");
    }else {
        println!("Yes");
    }
}
