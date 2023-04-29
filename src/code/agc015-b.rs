// https://atcoder.jp/contests/agc015/tasks/agc015_b

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
    let mut ans = 0;
    for i in 0..N {
        if S[i] == 'U' {
            ans += N - i - 1;
            ans += i * 2;
        }
        else {
            ans += i;
            ans += (N - i - 1) * 2;
        }
    }
    println!("{}", ans);
}