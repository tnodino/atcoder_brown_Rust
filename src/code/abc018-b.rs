// https://atcoder.jp/contests/abc018/tasks/abc018_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        N: usize,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    for _ in 0..N {
        input! {
            l: usize,
            r: usize,
        }
        let mut vec = Vec::new();
        vec.extend(S[..l-1].iter());
        vec.extend(S[l-1..r].iter().rev());
        vec.extend(S[r..].iter());
        S = vec;
    }
    println!("{}", S.iter().collect::<String>());
}