// https://atcoder.jp/contests/abc199/tasks/abc199_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
        Q: usize,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    let mut flg = false;
    for _ in 0..Q {
        input! {
            T: usize,
            mut A: usize,
            mut B: usize,
        }
        if T == 1 {
            A -= 1;
            B -= 1;
            if flg {
                if A < N {
                    A += N;
                }
                else {
                    A -= N;
                }
                if B < N {
                    B += N;
                }
                else {
                    B -= N;
                }
            }
            S.swap(A, B);
        }
        else {
            flg ^= true;
        }
    }
    let mut S = S.iter().collect::<String>();
    if flg {
        S = format!("{}{}", &S[N..], &S[..N]);
    }
    println!("{}", S);
}