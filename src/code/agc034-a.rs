// https://atcoder.jp/contests/agc034/tasks/agc034_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        A: usize,
        B: usize,
        C: usize,
        D: usize,
        S: String,
    }
    let A = A - 1;
    let B = B - 1;
    let C = C - 1;
    let D = D - 1;
    let S = S.chars().collect::<Vec<char>>();
    for i in A..C {
        if S[i] == '#' && S[i+1] == '#' {
            println!("No");
            return;
        }
    }
    for i in B..D {
        if S[i] == '#' && S[i+1] == '#' {
            println!("No");
            return;
        }
    }
    if C < D {
        println!("Yes");
    }
    else {
        for i in B-1..D {
            if S[i] == '.' && S[i+1] == '.' && S[i+2] == '.' {
                println!("Yes");
                return;
            }
        }
        println!("No");
    }
}