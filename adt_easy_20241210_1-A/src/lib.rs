pub fn add(n: i64, d: i64, v: Vec<i64>) -> i64 {
    let mut pre= 0;
    for i in 1..n{
        let submmit= v[i as usize] - v[(i-1) as usize];
        if submmit <= d{
            return v[i as usize];
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(4, 500, Vec::from([300, 900, 1300, 1700]));
        assert_eq!(result, 1300);
    }

    #[test]
    fn it_works2() {
        let result = add(4, 99, Vec::from([100, 200, 300, 400, 500]));
        assert_eq!(result, -1);
    }
}
