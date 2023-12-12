fn solution(a: usize, b: usize, c: usize, s: String) -> String{
    format!("{} {}", a+b+c, s)
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test1(){
        let a= 1usize;
        let b= 2usize; 
        let c= 3usize;
        let word= String::from("test");
        let expect= format!("{} {}", a+b+c, word);
        assert_eq!(expect, solution(1,2,3,String::from("test")));
    }
}