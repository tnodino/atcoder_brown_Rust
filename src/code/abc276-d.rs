// https://atcoder.jp/contests/abc276/tasks/abc276_d

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut a: [usize; N],
    }
    let mut g = 0;
    for i in 0..N {
        g = gcd(g, a[i])
    }
    let mut ans = 0;
    for i in 0..N {
        a[i] /= g;
        while a[i] % 2 == 0 {
            a[i] /= 2;
            ans += 1;
        }
        while a[i] % 3 == 0 {
            a[i] /= 3;
            ans += 1;
        }
        if a[i] > 1 {
            println!("-1");
            return;
        }
    }
    println!("{}", ans);
}