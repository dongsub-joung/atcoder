use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut split_input = input.split_whitespace();
    let a: usize = split_input.next().expect("Invalid input").parse().expect("Invalid number format");
    let b: usize = split_input.next().expect("Invalid input").parse().expect("Invalid number format");

    let number = a * b;

    if number % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}

