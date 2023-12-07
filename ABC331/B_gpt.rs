use std::io;

fn main() {
    // 標準入力から値を読み取る
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("読み取りエラー");

    // スペースで分割し、整数に変換する
    let mut values: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.trim().parse().expect("パースエラー"))
        .collect();

    let n = values[0]; // 必要な卵の数
    let s = values[1]; // 6個入りパックの価格
    let m = values[2]; // 8個入りパックの価格
    let l = values[3]; // 12個入りパックの価格

    // 最小の金額を計算する
    let min_cost = min_cost_for_eggs(n, s, m, l);
    println!("{}", min_cost);
}

// 最小の金額を計算する関数
fn min_cost_for_eggs(n: i32, s: i32, m: i32, l: i32) -> i32 {
    // DPテーブルを作成する
    let mut dp = vec![i32::MAX; (n + 1) as usize];
    dp[0] = 0;

    // ループを使って最小の金額を計算する
    for i in 1..=n {
        if i >= 6 {
            dp[i as usize] = dp[i as usize].min(dp[(i - 6) as usize] + s);
        }
        if i >= 8 {
            dp[i as usize] = dp[i as usize].min(dp[(i - 8) as usize] + m);
        }
        if i >= 12 {
            dp[i as usize] = dp[i as usize].min(dp[(i - 12) as usize] + l);
        }
    }

    dp[n as usize]
}

