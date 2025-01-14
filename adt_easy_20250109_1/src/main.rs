use std::io::{stdin, BufRead};

fn solution_a() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap()
        .parse::<usize>().unwrap();

    let mut s: Vec<String>= Vec::new(); 
    for _ in 0..n{
        let option= buf.next().unwrap().unwrap();
        s.push(option);
    }

    let vote_win= s.iter().filter(|&f| *f == "For".to_string()).count();

    if n == 1 && vote_win == 1{
        println!("Yes");
    }else if (n / 2) < vote_win {
        println!("Yes");
    }else{
        println!("No");
    }
}

fn main(){
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap()
        .parse::<usize>().unwrap();
    let str= buf.next().unwrap().unwrap();

    let mut chars: Vec<char>= str.chars().collect();

    let last_element= chars.pop().unwrap();

    println!("{}", last_element.to_string());
}