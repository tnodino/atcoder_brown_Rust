// https://atcoder.jp/contests/abc213/tasks/abc213_d

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn dfs(pos: usize, pre: usize, G: &Vec<Vec<usize>>, ans: &mut Vec<usize>) {
    ans.push(pos);
    for nxt in G[pos].iter() {
        if *nxt == pre {
            continue;
        }
        dfs(*nxt, pos, &G, ans);
        ans.push(pos);
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut G = vec![Vec::new(); N+1];
    for _ in 0..N-1 {
        input! {
            A: usize,
            B: usize,
        }
        G[A].push(B);
        G[B].push(A);
    }
    for i in 1..=N {
        G[i].sort();
    }
    let mut ans = Vec::new();
    dfs(1, 0, &G, &mut ans);
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}