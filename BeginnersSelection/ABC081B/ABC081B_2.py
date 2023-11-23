use std::io::stdin;

fn main() {
    let mut buff= String::new();
    stdin().read_line(&mut buff).expect("io err");
    let n= buff.clone();
    let n: u128= n.trim().parse().unwrap();

    buff.clear();
    stdin().read_line(&mut buff).expect("io err");
    let a= buff.clone();
    let mut Alist: Vec<usize>= Vec::new();
    let mut A= a.split_whitespace();
    for _ in 0..3{
        let string= A.next().unwrap();
        let per_a: usize= string.parse().unwrap();
        Alist.push(per_a);
    }

    let mut cntVector: Vec<usize>= Vec::new();
    for mut a in Alist{
        let mut cnt= 0;
        while a % 2 == 0 {
            a/= 2;
            cnt+= 1;
        }
        cntVector.push(cnt);
    } 

    let mut min= std::usize::MAX;
    for cnt in cntVector{
        if cnt < min{
            min= cnt;
        }
    }
    // 3 / 8 12 40
    println!("{}", min)
}
