// https://atcoder.jp/contests/abc187/tasks/abc187_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut cnt = 0;
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        cnt += A;
        vec.push(A * 2 + B);
    }
    vec.sort_by(|a, b| b.cmp(a));
    for i in 0..N {
        if cnt < vec[i] {
            println!("{}", i + 1);
            return;
        }
        cnt -= vec[i];
    }
}