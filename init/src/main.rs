fn solution(n: usize, d_vec: Vec<usize>) -> usize {
    
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1usize, solution(3usize, vec![15usize,15,15]));
    }

    #[test]
    fn test2() {
        assert_eq!(3usize, solution(4usize, vec![10usize,8,8,6]));
    }
}
