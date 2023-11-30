use std::io;

fn main() {
    let mut input= String::new();
    io::stdin().read_line(&mut input).expect("io err");

    let mut splite_input= input.split_whitespace();
    let n: usize= splite_input.next().expect("data err").parse().expect("parse err");
    let a: usize= splite_input.next().expect("data err").parse().expect("parse err");
    let b: usize= splite_input.next().expect("data err").parse().expect("parse err");

    let mut answer= 0;

    for i in 1..n{
        let mut num= i.to_string();
        let sumed: usize= num.chars()
            .map(|c| c.to_digit(10).unwrap() 
            as usize).sum();

        if a <= sumed && sumed <= b{
            answer+= i
        }
    }

    println!("{}", answer);
}
// 20 2 5
// 84

