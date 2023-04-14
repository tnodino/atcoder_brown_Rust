// https://atcoder.jp/contests/code-festival-2017-qualc/tasks/code_festival_2017_qualc_b

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = 0;
    for bit in 0..pow(3, N) {
        let mut x = bit;
        let mut flg = false;
        for i in 0..N {
            let a = match x % 3 {
                0 => A[i],
                1 => A[i] - 1,
                _ => A[i] + 1,
            };
            if a % 2 == 0 {
                flg = true;
            }
            x /= 3;
        }
        if flg {
            ans += 1;
        }
    }
    println!("{}", ans);
}