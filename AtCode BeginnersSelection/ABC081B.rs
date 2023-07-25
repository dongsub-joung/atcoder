fn solution(N: usize, A_vectors: Vec<usize>) -> usize{
    let mut cnt_vector= Vec::new();
    for mut A in A_vectors{
        let mut cnt= 0;
        while A % 2 == 0{
            A/= 2;
            cnt+=1;
        }
        cnt_vector.push(cnt);
    }

    let mut min= std::usize::MAX;
    for count in cnt_vector{
        if count < min{
            min= count;
        }
    }

    min
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(2usize, solution(3usize, vec![8,12,40]))
    }
}