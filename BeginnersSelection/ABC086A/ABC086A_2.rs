use std::io::stdin;
fn main() {
    let mut buf= String::new();
    stdin().read_line(&mut buf).expect("string err");
    let mut string= buf.split_whitespace();
    let a= string.next().expect("a value err");
    let b= string.next().expect("a value err");
    
    let a: usize= a.parse().unwrap();
    let b: usize= b.parse().unwrap();

    let number= a * b;
    
    // i.to_string().parse().unwrap(),

    if number % 2 == 0{
        println!("Even");
    }else {
        println!("Odd");
    }
}

