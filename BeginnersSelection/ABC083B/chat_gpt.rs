use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut split_input = input.split_whitespace();

    let n: usize = split_input.next().unwrap().parse().unwrap();
    let a: usize = split_input.next().unwrap().parse().unwrap();
    let b: usize = split_input.next().unwrap().parse().unwrap();

    let mut answer = 0;

    for i in 1..=n {
        let sum_of_digits = get_digit_sum(i);

        if sum_of_digits >= a && sum_of_digits <= b {
            answer += i;
        }
    }

    println!("{}", answer);
}

fn get_digit_sum(mut num: usize) -> usize {
    let mut sum = 0;

    while num > 0 {
        sum += num % 10;
        num /= 10;
    }

    sum
}

