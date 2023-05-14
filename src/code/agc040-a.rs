// https://atcoder.jp/contests/agc040/tasks/agc040_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let mut vec = vec![0; N+1];
    let S = S.chars().collect::<Vec<char>>();
    let mut cnt = 0;
    for i in 0..N {
        if S[i] == '<' {
            cnt += 1;
        }
        else {
            cnt = 0;
        }
        vec[i+1] = cnt;
    }
    cnt = 0;
    for i in (0..N).rev() {
        if S[i] == '>' {
            cnt += 1;
        }
        else {
            cnt = 0;
        }
        vec[i] = max(vec[i], cnt);
    }
    println!("{}", vec.iter().sum::<usize>());
}