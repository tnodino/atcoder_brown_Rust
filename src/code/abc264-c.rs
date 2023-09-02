// https://atcoder.jp/contests/abc264/tasks/abc264_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H1: usize,
        W1: usize,
        A: [[usize; W1]; H1],
        H2: usize,
        W2: usize,
        B: [[usize; W2]; H2],
    }
    for bit1 in 0..1<<H1 {
        for bit2 in 0..1<<W1 {
            let mut flg = vec![vec![true; W1]; H1];
            for i in 0..H1 {
                if bit1 & (1 << i) > 0 {
                    for j in 0..W1 {
                        flg[i][j] = false;
                    }
                }
            }
            for j in 0..W1 {
                if bit2 & (1 << j) > 0 {
                    for i in 0..H1 {
                        flg[i][j] = false;
                    }
                }
            }
            let mut C = Vec::new();
            for i in 0..H1 {
                let mut c = Vec::new();
                for j in 0..W1 {
                    if flg[i][j] {
                        c.push(A[i][j]);
                    }
                }
                if !c.is_empty() {
                    C.push(c);
                }
            }
            if C == B {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}