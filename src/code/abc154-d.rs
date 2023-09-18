// https://atcoder.jp/contests/abc154/tasks/abc154_d

use proconio::input;
use proconio::fastout;

fn max(x: f64, y: f64) -> f64 {
    return match x >= y {
        true => x,
        false => y,
    };
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        p: [f64; N],
    }
    let mut sum = 0.;
    for i in 0..K {
        sum += (p[i] + 1.) / 2.;
    }
    let mut ans = sum;
    for i in K..N {
        sum -= (p[i-K] + 1.) / 2.;
        sum += (p[i] + 1.) / 2.;
        ans = max(ans, sum);
    }
    println!("{}", ans);
}