// https://atcoder.jp/contests/arc159/tasks/arc159_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        _K: usize,
        mut a: [[usize; N]; N],
        Q: usize,
    }
    let INF: usize = 1 << 16;
    for i in 0..N {
        for j in 0..N {
            if a[i][j] == 0 {
                a[i][j] = INF;
            }
        }
    }
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                a[i][j] = min(a[i][j], a[i][k] + a[k][j]);
            }
        }
    }
    for _ in 0..Q {
        input! {
            s: usize,
            t: usize,
        }
        let s = (s - 1) % N;
        let t = (t - 1) % N;
        if a[s][t] == INF {
            println!("-1");
        }
        else {
            println!("{}", a[s][t]);
        }
    }
}