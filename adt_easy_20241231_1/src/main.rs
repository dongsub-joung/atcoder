use std::{collections::HashSet, io::{stdin, BufRead}};

fn main(){
    let std= stdin();
    let mut buf= std.lock().lines();
    let mut v: Vec<i32>= buf.next().unwrap().unwrap()
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap() )
            .collect();

    let mut hashset:HashSet<i32>= HashSet::new(); 
    let mut cnt= 0;
    let v_cloned= v.clone();
    
    for e in v{
        let amount= v_cloned.iter().filter(|&n| *n == e).count();


        if amount > 3{
            cnt= 2;
            println!("{}", cnt);
            return;
        }

        if amount > 2{
            cnt= 1;
            println!("{}", cnt);
            return;
        }

        if hashset.contains(&e){
            cnt+=1;
        }
        hashset.insert(e);
    }

    println!("{}", cnt);
}
