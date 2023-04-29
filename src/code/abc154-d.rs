// https://atcoder.jp/contests/abc154/tasks/abc154_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        p: [f64; N],
    }
    let mut imos = vec![0.; N+1];
    for i in 0..N {
        imos[i+1] += imos[i] + (1. + p[i]) / 2.;
    }
    let mut ans = 0.;
    for i in K..=N {
        if ans < imos[i] - imos[i-K] {
            ans = imos[i] - imos[i-K];
        }
    }
    println!("{}", ans);
}