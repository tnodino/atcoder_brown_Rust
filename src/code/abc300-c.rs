// https://atcoder.jp/contests/abc300/tasks/abc300_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let N = min(H, W);
    let mut C = Vec::new();
    for _ in 0..H {
        input! {
            c: String,
        }
        let c = c.chars().collect::<Vec<char>>();
        C.push(c);
    }
    let mut ans = vec![0; N];
    for i in 1..H-1 {
        for j in 1..W-1 {
            if C[i][j] == '#' && C[i-1][j-1] == '#' && C[i-1][j+1] == '#' && C[i+1][j-1] == '#' && C[i+1][j+1] == '#' {
                for k in 1..=N+1 {
                    if i < k || j < k {
                        ans[k-2] += 1;
                        break;
                    }
                    if C[i-k][j-k] != '#' {
                        ans[k-2] += 1;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}