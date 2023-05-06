// https://atcoder.jp/contests/abc072/tasks/arc082_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut p: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        if p[i] == i + 1 {
            ans += 1;
            if i < N - 1 {
                p[i+1] = p[i];
            }
        }
    }
    println!("{}", ans);
}