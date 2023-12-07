use std::io::{self, stdin};

fn main() {
    let mut buf= String::new();
    stdin().read_line(&mut buf).expect("io err");
    let v: Vec<usize>= buf.split_whitespace().map(|f| f.parse::<usize>().expect("parsing err")).collect();

    let mut n= v[0];
    let s= v[1];
    let m= v[2];
    let l= v[3];
    let mut s_cnt= 0usize;
    let mut m_cnt= 0usize;
    let mut l_cnt= 0usize;

    loop {
        if n >= 12{
            l_cnt= n / 12;
            n= n % 12;
        }else if n >= 8  && n < 12{
            m_cnt= n / 8;
            n= m % 8;
        }else if n >= 6 && n < 8{
            s_cnt= n / 6;
            n= n % 6
        }else if n < 6{
            s_cnt= s_cnt+1;
            break;
        }        
    }

    println!("{} {} {}", l_cnt, m_cnt, s_cnt);
    let answer= (l_cnt*l) + (m_cnt*m) + (s_cnt*s);

    // 16 120 150 200
    // 300
    println!("{}" , answer);
}

