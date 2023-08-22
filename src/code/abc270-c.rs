// https://atcoder.jp/contests/abc270/tasks/abc270_c

use proconio::input;
use proconio::fastout;

fn dfs(pos: usize, pre: usize, Y: usize, G: &Vec<Vec<usize>>, path: &mut Vec<usize>) {
    if pos == Y {
        println!("{}", path.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
        return;
    }
    for nxt in G[pos].iter() {
        if *nxt == pre {
            continue;
        }
        path.push(*nxt);
        dfs(*nxt, pos, Y, &G, path);
        path.pop();
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
    }
    let mut G = vec![Vec::new(); N+1];
    for _ in 0..N-1 {
        input! {
            U: usize,
            V: usize,
        }
        G[U].push(V);
        G[V].push(U);
    }
    let mut path = vec![X];
    dfs(X, 0, Y, &G, &mut path);
}