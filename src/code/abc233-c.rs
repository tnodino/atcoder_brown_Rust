// https://atcoder.jp/contests/abc233/tasks/abc233_c

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn dfs(x: usize, idx: usize, ans: &mut usize, X: usize, L: &Vec<usize>, a: &Vec<Vec<usize>>) {
    if idx == L.len() {
        if x == X {
            *ans += 1;
        }
        return;
    }
    for i in 0..L[idx] {
        if x <= X / a[idx][i] {
            dfs(x * a[idx][i], idx + 1, ans, X, &L, &a);
        }
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
    }
    let mut L = Vec::new();
    let mut a = Vec::new();
    for _ in 0..N {
        input! {
            l: usize,
            x: [usize; l],
        }
        L.push(l);
        a.push(x);
    }
    let mut ans = 0;
    dfs(1, 0, &mut ans, X, &L, &a);
    println!("{}", ans);
}