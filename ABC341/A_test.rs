use std::io::{stdin, BufRead};

pub fn solution(n: usize) -> String {
    let mut str_result= String::new();
    for i in 0..n*2+1{
        let j= i.clone() % 2;
        match j {
            0_usize => str_result.push_str("1"),
            1_usize => str_result.push_str("0"),
            _ => println!("Unvalid")
        }
    }
    
    str_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(4);
        assert_eq!(result, "101010101".to_string());
    }


    #[test]
    fn it_works2() {
        let result = solution(10);
        assert_eq!(result, "101010101010101010101".to_string());
    }    
}
