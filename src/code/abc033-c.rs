// https://atcoder.jp/contests/abc033/tasks/abc033_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let mut flg = true;
    if S[0] == '0' {
        flg = false;
    }
    let mut ans = 0;
    for i in (1..N).step_by(2) {
        if S[i] == '+' {
            if flg {
                ans += 1;
            }
            flg = true;
        }
        if S[i+1] == '0' {
            flg = false;
        }
    }
    if flg {
        ans += 1;
    }
    println!("{}", ans);
}