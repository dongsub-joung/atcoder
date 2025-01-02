use std::collections::HashSet;

pub fn solution_a(v: Vec<i32>) -> i32 {
    let mut hashset:HashSet<i32>= HashSet::new(); 
    let mut cnt= 0;
    let v_cloned= v.clone();
    
    for e in v{
        let amount= v_cloned.iter().filter(|&n| *n == e).count();
        
        if amount > 2{
            cnt= 1;
            return cnt;
        }
        if hashset.contains(&e){
            cnt+=1;
        }
        hashset.insert(e);
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution_a(Vec::from([2,1,2,1]));
        assert_eq!(result, 2);
    }
    
    #[test]
    fn it_works2() {
        let result = solution_a(Vec::from([4,4,4,1]));
        assert_eq!(result, 1);
    }
    
    #[test]
    fn it_works3() {
        let result = solution_a(Vec::from([1,2,3,4]));
        assert_eq!(result, 0);
    }
}
