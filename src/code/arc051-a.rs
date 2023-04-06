// https://atcoder.jp/contests/arc051/tasks/arc051_a

use proconio::input;
use proconio::fastout;
use libm::hypot;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x1: f64,
        y1: f64,
        r: f64,
        x2: f64,
        y2: f64,
        x3: f64,
        y3: f64,
    }
    if x2 <= x1 - r && x1 + r <= x3 &&
        y2 <= y1 - r && y1 + r <= y3 {
            println!("NO");
    }
    else {
        println!("YES");
    }
    if hypot((x2 - x1).abs(), (y2 - y1).abs()) <= r &&
        hypot((x3 - x1).abs(), (y2 - y1).abs()) <= r &&
        hypot((x2 - x1).abs(), (y3 - y1).abs()) <= r &&
        hypot((x3 - x1).abs(), (y3 - y1).abs()) <= r {
            println!("NO");
    }
    else {
        println!("YES");
    }
}