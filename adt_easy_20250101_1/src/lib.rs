pub fn solution_a(mut input: String) -> String {
    let mut result= String::new();
    
    let chars= input.chars().collect::<Vec<char>>();
    for i in (0..input.len()/2){
        if i%2!=0{
            result.push(chars[i+1]);
        }else{
            result.push(chars[i]);
        }
    }

    let mut cnt= 0;
    while cnt > input.len()/2 {

        cnt+=2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution_a("abcdef".to_string());
        assert_eq!(result, "badcfe".to_string());
    }
}

