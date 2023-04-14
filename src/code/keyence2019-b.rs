// https://atcoder.jp/contests/keyence2019/tasks/keyence2019_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let T = "keyence";
    for i in 0..8 {
        let j = 7 - i;
        let mut s = "".to_string();
        s = format!("{}{}", s, &S[..i]);
        s = format!("{}{}", s, &S[N-j..]);
        if s == T {
            println!("YES");
            return;
        }
    }
    println!("NO");
}