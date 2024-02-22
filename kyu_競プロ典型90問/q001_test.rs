pub fn solution(n: i32, l: i32, k: i32, a_vec: Vec<i32>) -> i32 {
    let mut part= -1;
    let mut part_two = l+1;

    while part-part_two > 1{
        let mid= (part + part_two) / 2;
        println!("{:?}", a_vec);
        if check(mid, a_vec.clone(), n, l, k) {
            part= mid;
        }else{
            part_two= mid;
        }
        println!("{:?}", a_vec);
    }

    part
}

fn check(mid: i32, a_vec: Vec<i32>, n: i32, l: i32, k: i32)-> bool{
    let mut num= 0;
    let mut pre= 0;
    for i in 0..n{
        let i= i as usize;
        if a_vec[i] - pre>= mid {
            num+= 1;
            pre= a_vec[i];
        }
    }
    if l - pre >= mid {
        num+= 1;
    }

    num>=k+1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n= 3;
        let l= 34;
        let k= 1;
        let a_vec: Vec<i32>= Vec::from([8, 13, 26]);
        let result = solution(n, l, k, a_vec);
        assert_eq!(result, 13);
    }
}

