fn solution(a: usize, b: usize, c: usize, x: usize) -> usize{
    let coins= vec![500,100, 50];
    let mut count= 0usize;
    for i in 0..a+1{
        for j in 0..b+1{
            for k in 0..c+1{
                if i * coins[0] + j * coins[1] + k * coins[2] == x{
                    count+=1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn testing(){
        let a= 2usize;
        let b= 2usize;
        let c= 2usize;
        let x= 100usize;
        assert_eq!(2usize, solution(a,b,c,x))
    }
}