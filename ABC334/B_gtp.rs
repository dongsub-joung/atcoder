use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let a: i64 = lines.next().unwrap().trim().parse().unwrap();
    let m: i64 = lines.next().unwrap().trim().parse().unwrap();
    let l: i64 = lines.next().unwrap().trim().parse().unwrap();
    let r: i64 = lines.next().unwrap().trim().parse().unwrap();

    let start = (l - a + m - 1) / m * m + a;
    let end = (r - a) / m * m + a;

    let count = (end - start) / m + 1;
    println!("{}", count);
}
