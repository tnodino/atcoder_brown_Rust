// https://atcoder.jp/contests/abc057/tasks/abc057_b

use proconio::input;
use proconio::fastout;
use std::isize::MAX;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..N {
        input! {
            x: isize,
            y: isize,
        }
        a.push(x);
        b.push(y);
    }
    let mut c = Vec::new();
    let mut d = Vec::new();
    for _ in 0..M {
        input! {
            x: isize,
            y: isize,
        }
        c.push(x);
        d.push(y);
    }
    for i in 0..N {
        let mut idx = 0;
        let mut dist = MAX;
        for j in 0..M {
            let x = (a[i] - c[j]).abs() + (b[i] - d[j]).abs();
            if x < dist {
                idx = j + 1;
                dist = x;
            }
        }
        println!("{}", idx);
    }
}