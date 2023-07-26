use std::io;

fn main() { 
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    // let a: Vec<usize> = buf.split_whitespace().map(|s| s.trim().parse()::<F>.unwrap().collect::<Vec<_>>());
    let mut inputed= buf.split_whitespace();
    
    let a= inputed.next().unwrap();
    let b= inputed.next().unwrap();
    let c= inputed.next().unwrap();
    
    let a: usize= a.parse().unwrap();
    let b: usize= b.parse().unwrap();
    let c: usize= c.parse().unwrap();

    if a + b + c == 17usize{
        print!("YES");
    }else{
        print!("NO");
    }
}

