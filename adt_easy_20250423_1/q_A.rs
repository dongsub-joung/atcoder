use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let str= buf.next().unwrap().unwrap();
    let chars= str.as_bytes();
    let c= chars[0] as char;

    let result_udp= c.to_string().push_str("UDP");
    println!("{}", result_udp);
}
