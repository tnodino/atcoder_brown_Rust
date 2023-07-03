// https://atcoder.jp/contests/arc115/tasks/arc115_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        C: [[usize; N]; N],
    }
    let mut mi: usize = 1<<60;
    for i in 0..N {
        mi = min(mi, C[i][0]);
    }
    let mut A = vec![0; N];
    for i in 0..N {
        A[i] = C[i][0] - mi;
    }
    let mut B = vec![0; N];
    for j in 0..N {
        if C[0][j] < A[0] {
            println!("No");
            return;
        }
        B[j] = C[0][j] - A[0];
    }
    for i in 0..N {
        for j in 0..N {
            if A[i] + B[j] != C[i][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
    println!("{}", A.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    println!("{}", B.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}