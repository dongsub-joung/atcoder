use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let having: Vec<i32> = buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let prices: Vec<i32> = buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    
    let normal_price= having[1];
    let discount_price= having[2];

    let mut total= 0;

    let mut next_lvl_price= false;
    for price in prices.clone() {
        if price > normal_price {
            next_lvl_price= true;
            break;
        }
    }

    let mut cheap=i32::MAX;
    if !next_lvl_price{
        for price in prices{
            if price < cheap{
                cheap= price;
            }
        }
        let sum= cheap + discount_price;   
        println!("{}", sum);    
    }else{
        println!("{}", normal_price);
    }
    
}
