use std::{io::{stdin, BufRead}, fmt::format};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let input= buf.next().unwrap().unwrap();
    let n: usize = (&input).trim().parse().expect("Please enter a valid number");

    if n >= 1 && n <= 9 {
        let mut result = String::new();

        for _ in 0..n {
            result.push_str(&n.to_string());
        }

        println!("{}", result);
    } else {
        println!("Nは1以上9以下の整数である必要があります");
    }
}
