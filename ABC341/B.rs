use std::io::{stdin, BufRead};

fn main(){
    let std= stdin();
    let mut buf= std.lock().lines();
    let n: usize= buf.next().unwrap().unwrap().parse().unwrap();

    let mut a_vec: Vec<usize>= buf.next().unwrap().unwrap().trim().split_whitespace().map(|f| f.parse().unwrap()).collect();
    
    let mut st: Vec<(usize, usize)>= Vec::new();
    for _ in 0..n-1 {
        let st_vec: Vec<usize>= buf.next().unwrap().unwrap().trim().split_whitespace().map(|f| f.parse().unwrap()).collect();
        let (s, t)= (st_vec[0], st_vec[1]);
        st.push((s,t));
    }

    loop {
        let max= a_vec.iter().max().unwrap().clone();
        let max_index= a_vec.iter().position(|&f| f == max).unwrap();
        let i= max_index+1;

        let (s,t)= st[i];
        if a_vec[max_index] > s {
            a_vec[max_index] -= s;
            a_vec[max_index+1] += t;
        }else{
            break;
        }
    }
    
    println!("{}", a_vec.iter().max().unwrap().clone());
}
