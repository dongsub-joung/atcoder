use std::io::stdin;

fn main() {
    let mut answer= 0;
    let mut buff= String::new();
    
    stdin().read_line(&mut buff).expect("io err");
    let charVec:Vec<char>= buff.chars().collect();
    
    for c in charVec{
        if c == '1'{
            answer+= 1;
        }
    }

    println!("{}", answer);
}
