// https://atcoder.jp/contests/abc183/tasks/abc183_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: isize,
    }
    let M = 200_000;
    let mut imos = vec![0; M+1];
    for _ in 0..N {
        input! {
            S: usize,
            T: usize,
            P: isize,
        }
        imos[S] += P;
        imos[T] -= P;
    }
    for i in 0..M {
        if imos[i] > W {
            println!("No");
            return;
        }
        imos[i+1] += imos[i];
    }
    println!("Yes");
}