// https://atcoder.jp/contests/arc141/tasks/arc141_a

use proconio::input;
use std::cmp::max;

#[allow(non_snake_case)]
fn solve(pow10: &Vec<usize>) {
    input! {
        N: usize,
    }
    let mut k = 0;
    for i in 1..=18 {
        if N / pow10[i] == 0 {
            k = i;
            break;
        }
    }
    let mut ans = "9".repeat(k-1).parse::<usize>().unwrap();
    for i in 1..k {
        if k % i != 0 {
            continue;
        }
        let m = k / i;
        let x = N / pow10[k-i];
        let mut n = 0;
        for _ in 0..m {
            n *= pow10[i];
            n += x;
        }
        if n <= N {
            ans = max(ans, n);
        }
        let x = N / pow10[k-i] - 1;
        let mut n = 0;
        for _ in 0..m {
            n *= pow10[i];
            n += x;
        }
        if n <= N {
            ans = max(ans, n);
        }
    }
    println!("{}", ans);
}

#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    let N = 18;
    let mut pow10 = vec![1; N+1];
    for i in 1..=N {
        pow10[i] = pow10[i-1] * 10;
    }
    for _ in 0..T {
        solve(&pow10);
    }
}