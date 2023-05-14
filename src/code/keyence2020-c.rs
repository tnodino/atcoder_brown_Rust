// https://atcoder.jp/contests/keyence2020/tasks/keyence2020_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        S: usize,
    }
    let M = 1_000_000_000;
    let mut A = Vec::new();
    for _ in 0..K {
        A.push(S);
    }
    for _ in K..N {
        A.push((S % M) + 1);
    }
    println!("{}", A.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}