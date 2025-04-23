use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap().parse::<i32>().unwrap();

    let mut idx= 0;
    let mut result_cnt= 0;
    while n > idx {
        let name= buf.next().unwrap().unwrap();
        if name.contains("Takahashi") {
            result_cnt+=1;
        }
        idx+=1;
    }

    println!("{}", result_cnt);
}
