use std::{collections::{hash_map, HashMap}, io::{stdin, BufRead}};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let input_v: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap() )
        .collect();
    
    let mut hashmap_one: HashMap<i32, i32>=  HashMap::new();
    let mut hashmap_two: HashMap<i32, i32>=  HashMap::new();
    for e in input_v.clone() {
        hashmap_one.insert(e, 1);
    }
    for e in input_v.clone() {
        if *(hashmap_one.get(&e).unwrap()) == 1 {
            hashmap_two.insert(e, 2);
        }
    }
    

    println!("{:#?}", hashmap_two);

    let mut result= "";
    for e in input_v {
        let value= *(hashmap_two.get(&e).unwrap());
        if value == 2{
            result= "Yes";
        }
    }

    println!("{}", result);
}
