// https://atcoder.jp/contests/adt_easy_20241217_1/tasks/abc252_a
pub fn solution_a(n: u64) -> String {
    let c= char::from_u32(n as u32).unwrap();

    c.to_string()    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = solution_a(97);
        assert_eq!(result, "a".to_string());
    }

    #[test]
    fn it_works2() {
        let result = solution_a(122);
        assert_eq!(result, "z".to_string());
    }
}


// https://atcoder.jp/contests/adt_easy_20241217_1/tasks/abc304_a
pub fn solution_b(n: i32, s_v: Vec<(&str, i32)>) -> Vec<&str> {
    let mut result_v= Vec::new();
    let min_age= s_v.iter().min().unwrap().1;
    let mut ages: Vec<i32>= Vec::new();

    for (k,v) in s_v.clone() {
        ages.push(v);
    }

    let min= ages.iter().min().unwrap();

    let mut idx= 0;
    for (i,(k, v)) in s_v.clone().iter().enumerate(){
        if min == v{
            idx= i;
        }
    }

    for i in idx..(n as usize){
        let (k, v)= s_v[i];
        result_v.push(k);
    }

    for i in 0..idx{
        let (k, v)= s_v[i];
        result_v.push(k);
    }
    
    result_v
}


#[cfg(test)]
mod tests_b {
    use super::*;

    #[test]
    fn it_works1() {
        let result = solution_b(5,Vec::from([("alice", 31)
            , ("bob", 41)
            ,("carol", 5)
            ,("dave", 92)
            ,("ellen", 65)]));
        assert_eq!(result, Vec::from(["carol"
            ,"dave"
            ,"ellen"
            ,"alice"
            ,"bob"]));
    }
}