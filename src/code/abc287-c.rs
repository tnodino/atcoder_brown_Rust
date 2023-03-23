// https://atcoder.jp/contests/abc287/tasks/abc287_c

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn dfs(pos: usize, G: &Vec<Vec<usize>>, flg: &mut Vec<bool>) {
    flg[pos] = true;
    for nxt in G[pos].iter() {
        if flg[*nxt] {
            continue;
        }
        dfs(*nxt, G, flg);
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..M {
        input! {
            u: usize,
            v: usize,
        }
        G[u-1].push(v-1);
        G[v-1].push(u-1);
    }
    if N - 1 != M {
        println!("No");
        return;
    }
    for i in 0..N {
        if G[i].len() > 2 {
            println!("No");
            return;
        }
    }
    let mut flg = vec![false; N];
    dfs(0, &G, &mut flg);
    if flg.iter().all(|&x| x) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}