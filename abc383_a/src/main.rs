use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap()
        .parse::<i32>().unwrap();
    let mut t_times: Vec<Vec<i32>>= Vec::new();
    for _ in 0.. n {
        let string= buf.next().unwrap().unwrap();
        let v: Vec<i32>= string
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap() )
            .collect();

        t_times.push(v);
    }

    let mut remained= 0;
    let mut last= 0;
    for i in 0..n {
        remained= 0.max(remained- (t_times[i as usize][0] -last));
        remained+= t_times[i as usize][1];
        last= t_times[i as usize][1].clone();
    }


    // for (i, v) in t_times.clone().iter().enumerate() {
    //     if i == 0 {
    //         remained+= (v[1]-v[0] + 0);

    //     }else{
    //         remained+= (v[1]-v[0] + t_times[i-1][0]);
    //     }
    // }

    println!("{}", remained);
}
