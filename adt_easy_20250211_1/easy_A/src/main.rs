use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let n= std.lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    let chars: Vec<char>= Vec::from(['A', 'B', 'C']);
    let nokori= n % 3;
    let times= (n as f32) / 3.0;

    let mut result_string= String::new();
  
    let abc= "ABC".to_string();
    if times < 1.0 && nokori >= 1 {
        println!("{}", &abc[0..nokori as usize]);
    }else if times == 1.0 && nokori == 0 {
        println!("ABC");
    }else if times != 1.0 {
        for _ in 0..(times as usize){
            result_string.push_str("ABC");
        }
        result_string.push_str(&abc[0..nokori as usize]);
    }

    println!("{}", result_string);
}
