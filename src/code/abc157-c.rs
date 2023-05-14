// https://atcoder.jp/contests/abc157/tasks/abc157_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut vec = vec![10; N];
    for _ in 0..M {
        input! {
            s: usize,
            c: usize,
        }
        if vec[s-1] != 10 && vec[s-1] != c {
            println!("-1");
            return;
        }
        vec[s-1] = c;
    }
    for i in 0..N {
        if i == 0 {
            if N != 1 && vec[i] == 0 {
                println!("-1");
                return;
            }
            if vec[i] == 10 {
                if N == 1 {
                    vec[i] = 0;
                }
                else {
                    vec[i] = 1;
                }
            }
        }
        else {
            if vec[i] == 10 {
                vec[i] = 0;
            }
        }
    }
    let mut ans = 0;
    for i in 0..N {
        ans *= 10;
        ans += vec[i];
    }
    println!("{}", ans);
}