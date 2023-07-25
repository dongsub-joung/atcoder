fn solution(s: &'static str)-> usize{
    let char_vec: Vec<char>= s.chars().collect();
    let mut ans= 0;
    for e in char_vec{
        if e == '1'{
            ans+=1;
        }
    }

    ans
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(2, solution("101"))
    }
}
