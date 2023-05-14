// https://atcoder.jp/contests/agc039/tasks/agc039_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        K: usize,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let mut vec  = Vec::new();
    let mut cnt = 1;
    for i in 1..N {
        if S[i-1] != S[i] {
            vec.push(cnt);
            cnt = 0;
        }
        cnt += 1;
    }
    vec.push(cnt);
    if vec.len() == 1 {
        println!("{}", N * K / 2);
        return;
    }
    let M = vec.len();
    let mut ans: usize = 0;
    for i in 1..M-1 {
        ans += vec[i] / 2 * K;
    }
    if S[0] == S[N-1] {
        ans += vec[0] / 2;
        ans += vec[M-1] / 2;
        ans += (vec[0] + vec[M-1]) / 2 * (K - 1);
    }
    else {
        ans += vec[0] / 2 * K;
        ans += vec[M-1] / 2 * K;
    }
    println!("{}", ans);
}