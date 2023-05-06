// https://atcoder.jp/contests/agc022/tasks/agc022_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    if S.len() <= 25 {
        for i in 0..26 {
            let c = (i as u8 + 97) as char;
            if !S.contains(&c) {
                println!("{}{}", S.iter().map(|&x| x.to_string()).collect::<String>(), c);
                return;
            }
        }
    }
    else {
        for i in (0..25).rev() {
            if S[i] < S[i+1] {
                let m = S[i] as usize - 97;
                let S = &S[..i];
                for k in m+1..26 {
                    let c = (k as u8 + 97) as char;
                    if !S.contains(&c) {
                        println!("{}{}", S.iter().map(|&x| x.to_string()).collect::<String>(), c);
                        return;
                    }
                }
            }
        }
    }
    println!("-1");
}