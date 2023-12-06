fn solution(mut n: usize, mut v: Vec<usize>) ->usize {
    v.sort();

    let mut idx= 0;
    let mut answer= 0;

    while true{
        if idx % 2 ==0 {
            let value= v.pop();
            match value{
                Some(e) => answer+= value.unwrap_or(0),
                None => print!("None"),
            }
        }else{
           let value= v.pop();
           match value{
            Some(e) => answer-= value.unwrap_or(0),
                None => print!("None"),
            }
        }

        idx+=1;
        n-= 1;
        
        if n == 0 {
            break;
        }
    }

    answer
}

#[cfg(test)]
mod test{
    use super::*;
    
    // 088B
    #[test]
    fn testing(){
        let n= 2usize;
        let v= vec![3, 1];
        assert_eq!(2usize, solution(n,v))
    }

    #[test]
    fn testing2(){
        let n= 3usize;
        let v= vec![2, 7, 4];
        assert_eq!(5usize, solution(n,v))
    }
}
