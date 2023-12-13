pub fn init(n: i32, y: i32) -> (i32, i32, i32) {
    let mut x= 0; let mut y_cnt= 0; let mut z= 0;

    let mut money= false;

    for x in 0..=n{
        for y_cnt in 0..=n-x{
            z= n - x - y_cnt;
            let total= 100000 * x + 5000 * y_cnt + 1000 *z;

            if total == y{
                money= true;
                return (x, y_cnt, z);
            }
        }
        if money{
            break;
        }
    }
    if !money{
        return (-1,-1,-1)
    }

    (x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = init(9, 45000);
        assert_eq!(result, (4,0,5));
    }

    #[test]
    fn it_works2() {
        let result = init(20, 196000);
        assert_eq!(result, (-1,-1,-1));
    }
}
