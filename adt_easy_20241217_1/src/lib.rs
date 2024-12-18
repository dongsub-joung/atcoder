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