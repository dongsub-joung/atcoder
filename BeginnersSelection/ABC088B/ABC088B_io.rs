use std::io::{self, BufRead};

fn main(){

    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let mut n: usize = input
        .next()
        .unwrap()
        .expect("Couldn't read line")
        .trim()
        .parse()
        .expect("Parsing error");

    let mut numbers: Vec<usize> = input
        .next()
        .unwrap()
        .expect("Couldn't read line")
        .split_whitespace()
        .map(|s| s.parse().expect("Parsing error"))
        .collect();

    numbers.sort();

    let mut idx= 0;
    let mut answer= 0;

    loop{
        if idx % 2 ==0 {
            let value= numbers.pop();
            match value{
                Some(e) => answer+= value.unwrap_or(0),
                None => print!("None"),
            }
        }else{
           let value= numbers.pop();
           match value{
            Some(e) => answer-= value.unwrap_or(0),
                None => print!("None"),
            }
        }

        idx+=1;
        n-= 1;
        
        if n == 0 {
            break;
        }
    }

    println!("{}", answer);
}

