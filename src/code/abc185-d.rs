// https://atcoder.jp/contests/abc185/tasks/abc185_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize; M],
    }
    if N == M {
        println!("0");
        return;
    }
    A.sort();
    A.push(N + 1);
    let mut W = Vec::new();
    let mut x = 1;
    for a in A {
        if a > x {
            W.push(a - x);
        }
        x = a + 1;
    }
    let mi = *W.iter().min().unwrap();
    let mut ans = 0;
    for w in W {
        ans += (w + mi - 1) / mi;
    }
    println!("{}", ans);
}