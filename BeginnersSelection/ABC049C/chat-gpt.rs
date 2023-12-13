use std::io;

fn can_form_empty_string(s: &str) -> String {
    let mut s = s.to_string();
    while !s.is_empty() {
        if s.ends_with("dreamer") {
            s.truncate(s.len() - 7);
        } else if s.ends_with("dream") {
            s.truncate(s.len() - 5);
        } else if s.ends_with("eraser") {
            s.truncate(s.len() - 6);
        } else if s.ends_with("erase") {
            s.truncate(s.len() - 5);
        } else {
            return "NO".to_string();
        }
    }
    return "YES".to_string();
}

fn main() {
    let mut input = String::new();

    // 標準入力から文字列を受け取る
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();

    // 空文字列にできるか判定する
    let result = can_form_empty_string(s);
    println!("{}", result);
}

