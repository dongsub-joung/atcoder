use std::io::{slef, stdin, BufRead};

fn main{
    // (n: i32, y: i32)
    let std= stdin();
    let mut inputs= std.lock().lines();
    let numbers: Vec<i32>= inputs.next().unwrap().unwrap().split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();

    let mut x= numbers[0]; 
    let mut y_cnt=  numbers[1]; 
    let mut z=  numbers[2];

    let mut money= false;

    for x in 0..=n{
        for y_cnt in 0..=n-x{
            z= n - x - y_cnt;
            let total= 100000 * x + 5000 * y_cnt + 1000 *z;

            if total == y{
                money= true;
                return (x, y_cnt, z);
            }
        }
        if money{
            break;
        }
    }
    if !money{
        println!("-1 -1 -1");
    }

    println!("{} {} {}", x, y_cnt, z);
}
