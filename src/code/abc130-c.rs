// https://atcoder.jp/contests/abc130/tasks/abc130_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        W: f64,
        H: f64,
        x: f64,
        y: f64,
    }
    let area = W * H / 2.;
    if W / 2. == x && H / 2. == y {
        println!("{} 1", area);
    }
    else {
        println!("{} 0", area);
    }
}