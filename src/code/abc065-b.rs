// https://atcoder.jp/contests/abc065/tasks/abc065_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut flg = vec![true; N];
    let mut idx = 0;
    let mut ans = 0;
    while flg[idx] {
        flg[idx] = false;
        idx = a[idx] - 1;
        ans += 1;
        if idx == 1 {
            println!("{}", ans);
            return;
        }
    }
    println!("-1");
}