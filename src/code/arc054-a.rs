// https://atcoder.jp/contests/arc054/tasks/arc054_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        L: f64,
        X: f64,
        Y: f64,
        S: f64,
        D: f64,
    }
    let dir;
    if S <= D {
        dir = D - S;
    }
    else {
        dir = D + L - S;
    }
    let rev_dir = L - dir;
    if X < Y {
        let ans1 = dir / (X + Y);
        let ans2 = rev_dir / (Y - X);
        if ans1 < ans2 {
            println!("{}", ans1);
        }
        else {
            println!("{}", ans2);
        }
    }
    else {
        println!("{}", dir / (X + Y));
    }
}