// https://atcoder.jp/contests/arc145/tasks/arc145_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    if S[0] == 'A' && S[N-1] == 'B' {
        println!("No");
    }
    else if N == 2 && S[0] == 'B' && S[1] == 'A' {
        println!("No");
    }
    else {
        println!("Yes");
    }
}