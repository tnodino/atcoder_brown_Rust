// https://atcoder.jp/contests/agc028/tasks/agc028_a

use proconio::input;
use proconio::fastout;
use num::integer::lcm;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        S: String,
        T: String,
    }
    let L = lcm(N, M);
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    let mut t = 0;
    for s in 0..N {
        let x = (L / N) * s;
        while (L / M) * t < x {
            t += 1;
        }
        let y = (L / M) * t;
        if x == y {
            if S[s] != T[t] {
                println!("-1");
                return;
            }
        }
    }
    println!("{}", L);
}