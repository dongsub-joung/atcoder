fn main() {
    let mut s= "Hello, world! dog".to_string();
    // s.truncate(4); 
    let b= s.ends_with("dog");
    
    println!("{}", s);
    println!("{}", b);
}
