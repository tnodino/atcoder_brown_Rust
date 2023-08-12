// https://atcoder.jp/contests/abc258/tasks/abc258_c

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
    let mut cnt = 0;
    for _ in 0..Q {
        input! {
            q: usize,
            x: usize,
        }
        if q == 1 {
            cnt = (cnt + N - x) % N;
        }
        else {
            let idx = (cnt + x - 1) % N;
            println!("{}", S[idx]);
        }
    }
}