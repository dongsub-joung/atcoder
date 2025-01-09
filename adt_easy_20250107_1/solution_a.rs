//   8/10
// 2025-01-09 15:32:49 	A - Tomorrow 	kizari 	Rust (rustc 1.70.0) 	0 	885 Byte 	
// 1 ms	2068 KB 	詳細
use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let limit: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();
    let present_days: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();

    let max_mm= limit[0];
    let max_dd= limit[1];

    let mut next_year= present_days[0].clone();
    let mut next_month= present_days[1].clone();
    let mut next_day= present_days[2].clone();
    if present_days[2] >= max_dd {
        next_day= 1;
        next_month+=1;
    }else{
        next_day+= 1;
    }

    if present_days[1] >= max_mm {
        next_month= 1;
        next_year+=1;
    }

    println!("{} {} {}", next_year, next_month, next_day);
}