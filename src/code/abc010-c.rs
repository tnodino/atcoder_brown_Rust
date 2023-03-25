// https://atcoder.jp/contests/abc010/tasks/abc010_3

use proconio::input;
use proconio::fastout;
use libm::hypot;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        txa: f64,
        tya: f64,
        txb: f64,
        tyb: f64,
        T: f64,
        V: f64,
        n: usize,
    }
    for _ in 0..n {
        input! {
            x: f64,
            y: f64,
        }
        if hypot((txa - x).abs(), (tya - y).abs()) + hypot((txb - x).abs(), (tyb - y).abs()) <= T * V {
            println!("YES");
            return;
        }
    }
    println!("NO");
}