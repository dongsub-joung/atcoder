pub fn solution(n: usize, mut a_vec: Vec<usize>, st: Vec<(usize, usize)>) -> usize {
    loop {
        let max= a_vec.iter().max().unwrap().clone();
        let max_index= a_vec.iter().position(|&f| f == max).unwrap();
        let i= max_index+1;

        let (s,t)= st[i];
        if a_vec[max_index] > s {
            a_vec[max_index] -= s;
            a_vec[max_index+1] += t;
        }else{
            break;
        }
    }
    
    a_vec.iter().max().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n= 4;
        let a_vec= Vec::from([5,7,0,3]);
        let st_vec= Vec::from([(2,2), (4,3), (5,2)]);

        let result = solution(n, a_vec, st_vec);
        assert_eq!(result, 5);
    }

    #[test]
    fn it_works2() {
        let n= 10;
        let a_vec= Vec::from([32, 6, 46, 9, 37, 8, 33, 14, 31,5]);
        let st_vec= Vec::from([(5,5), (3,1), (4,3), (2,2), (3,2), (3,2), (4,4), (3,3), (3,1)]);
        let result = solution(n, a_vec, st_vec);
        assert_eq!(result, 45);
    }
}
