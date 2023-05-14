// https://atcoder.jp/contests/abc122/tasks/abc122_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut cnt = vec![0; N+1];
    for i in 1..N {
        cnt[i+1] += cnt[i];
        if S[i-1] == 'A' && S[i] == 'C' {
            cnt[i+1] += 1;
        }
    }
    for _ in 0..Q {
        input! {
            l: usize,
            r: usize,
        }
        println!("{}", cnt[r] - cnt[l]);
    }
}