// https://atcoder.jp/contests/caddi2018b/tasks/caddi2018_a

use proconio::input;
use proconio::fastout;
use libm::sqrt;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut P: usize,
    }
    let M = sqrt(P as f64) as usize;
    let mut ans = 1;
    for i in 2..=M {
        let mut cnt = 0;
        while P % i == 0 {
            cnt += 1;
            P /= i;
        }
        ans *= pow(i, cnt / N);
    }
    if N == 1 {
        ans *= P;
    }
    println!("{}", ans);
}