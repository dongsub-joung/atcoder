fn solution(a: usize, b: usize) -> &'static str{
    if (a*b) % 2 == 0{
        "Even"
    }else{
        "Odd"
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test1(){
        assert_eq!("Even", solution(3,4));
    }

    #[test]
    fn test2(){
        assert_eq!("Odd", solution(1,21));
    }
}