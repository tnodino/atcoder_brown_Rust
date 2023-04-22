// https://atcoder.jp/contests/abc173/tasks/abc173_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.sort_by(|a, b| b.cmp(a));
    let mut cnt = 1;
    let mut idx = 0;
    let mut ans = 0;
    for _ in 1..N {
        ans += A[idx];
        cnt += 1;
        if cnt == 2 {
            cnt = 0;
            idx += 1;
        }
    }
    println!("{}", ans);
}