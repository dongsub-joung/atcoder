use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut listing = Vec::new();

    for line in stdin.lock().lines().take(4) {
        let line = line.expect("Failed to read line");
        let mut numbers = line.split_whitespace();

        if let Some(number_str) = numbers.next() {
            if let Ok(number) = number_str.parse::<usize>() {
                listing.push(number);
            }
        }
    }

    let a= listing[0];
    let b= listing[1];
    let c= listing[2];
    let x= listing[3];

    let coins= vec![500, 100, 50];
    let mut count= 0usize;
    for i in 0..a+1{
        for j in 0..b+1{
            for k in 0..c+1{
                if i * coins[0] + j * coins[1] + k * coins[2] == x{
                    count+=1;
                }
            }
        }
    }



    // 30,40,50,6000 => 213

    println!("{}", count)
}
