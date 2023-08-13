// https://atcoder.jp/contests/abc230/tasks/abc230_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        A: usize,
        B: usize,
        P: usize,
        Q: usize,
        R: usize,
        S: usize,
    }
    let H = Q - P + 1;
    let W = S - R + 1;
    for i in 0..H {
        let mut res = vec!['.'; W];
        for j in 0..W {
            let x = i + P;
            let y = j + R;
            if x <= A && y <= B && A - x == B - y {
                res[j] = '#';
            }
            if x <= A && B <= y && A - x == y - B {
                res[j] = '#';
            }
            if A <= x && y <= B && x - A == B - y {
                res[j] = '#';
            }
            if A <= x && B <= y && x - A == y - B {
                res[j] = '#';
            }
        }
        println!("{}", res.iter().collect::<String>());
    }
}