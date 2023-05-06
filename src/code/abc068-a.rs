// https://atcoder.jp/contests/abc068/tasks/arc079_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut flg1 = vec![false; N];
    let mut flg2 = vec![false; N];
    for _ in 0..M {
        input! {
            a: usize,
            b: usize,
        }
        if a == 1 {
            flg1[b-1] = true;
        }
        if b == N {
            flg2[a-1] = true;
        }
    }
    for i in 0..N {
        if flg1[i] && flg2[i] {
            println!("POSSIBLE");
            return;
        }
    }
    println!("IMPOSSIBLE");
}