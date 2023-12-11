pub fn add(n: usize, v: Vec<usize>) -> usize {
    let mut answer= 0;
    let v: Vec<usize>= v.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();
    answer= v.len();
    answer    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(4, vec![10 ,8, 8, 6]);
        assert_eq!(result, 3);
    }


    #[test]
    fn it_works2() {
        let result = add(3, vec![15,15,15]);
        assert_eq!(result, 1);
    }
}
