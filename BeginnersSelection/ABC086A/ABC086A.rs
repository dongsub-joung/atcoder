use std::io::stdin;

fn main() {
    let mut buf= String::new();
    stdin().read_line(&mut buf).expect("string err");
    
    let listing= buf.chars();
    let mut number= 1usize;

    for i in listing{
        let n: usize= match i {
            '1' => i.to_string().parse().unwrap(),
            '2' => i.to_string().parse().unwrap(),
            '3' => i.to_string().parse().unwrap(),
            '4' => i.to_string().parse().unwrap(),
            '5' => i.to_string().parse().unwrap(),
            '6' => i.to_string().parse().unwrap(),
            '7' => i.to_string().parse().unwrap(),
            '8' => i.to_string().parse().unwrap(),
            '9' => i.to_string().parse().unwrap(),
            '0' => i.to_string().parse().unwrap(),
            '\n' => continue,
            _ => continue
        };
        
        number*= n;
    }

    if number % 2 == 0{
        println!("Even");
    }else {
        println!("Odd");
    }
}

